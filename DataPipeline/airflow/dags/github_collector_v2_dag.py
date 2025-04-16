"""
Airflow DAG for GitHub Collector V2

This DAG collects GitHub organization data in three parallel streams:
1. Repository Data: Collects detailed repository information including PRs, issues, and commits
2. Team Data: Collects team structure, members, and repository assignments
3. Member Data: Collects detailed user information and contribution metrics

The data is stored in a date-based directory structure for easy tracking and governance.
"""

import asyncio
from datetime import datetime, timedelta
from pathlib import Path
from dotenv import load_dotenv
import boto3
import shutil
from typing import Any

from airflow import DAG
from airflow.operators.python import PythonOperator
from airflow.models import Variable
from airflow.hooks.base import BaseHook
from airflow.exceptions import AirflowException

from src.github_collector_v2.collector import GitHubCollector
from src.utils.loggers import get_logger
from src.utils.connection_checker import create_connection_manager
from plugins.path_utils_plugin import (
    get_data_file_paths,
    update_symlink,
)

# Set up logging
logger = get_logger("github_collector_v2.dag")

# DAG Constants
DAG_ID = "github_collector_v2"
DAG_DESCRIPTION = (
    "Collect GitHub organization data including repositories, teams, and users"
)
DAG_SCHEDULE = timedelta(days=1)
DAG_START_DATE = datetime(2024, 1, 1)
DAG_TAGS = ["github", "data_collection"]

# File path constants
BASE_DIR = Path(Variable.get("GITHUB_DATA_DIR", default_var="data/raw/github"))

# Default arguments
default_args = {
    "owner": "airflow",
    "depends_on_past": False,
    "email_on_failure": False,
    "email_on_retry": False,
    "retries": 1,
    "retry_delay": timedelta(minutes=5),
}

# Load environment variables
load_dotenv()


def check_connections(collector: GitHubCollector, **context):
    """Task to check all service connections"""
    try:
        # Create and configure connection manager with Airflow connections
        manager = create_connection_manager(
            github_collector=collector,
            redis_conn_id="redis_default",
            aws_conn_id="aws_default",
        )

        # Run all checks in parallel with 30 second timeout
        results = asyncio.run(manager.check_all(timeout=30.0))

        # Store results in XCom for downstream tasks
        context["task_instance"].xcom_push(
            key="connection_check_results", value=results
        )

        # Log detailed results
        logger.info("Connection Check Results:")
        logger.info("------------------------")

        # Log successful checks
        if results["success"]:
            logger.info("\nSuccessful Connections:")
            for result in results["success"]:
                service = result["service"].upper()
                details = result.get("details", {})

                if service == "REDIS":
                    logger.info(
                        "Redis: %s:%s (DB: %s) Version: %s, Memory: %s",
                        details["connection"]["host"],
                        details["connection"]["port"],
                        details["connection"]["db"],
                        details["stats"]["version"],
                        details["stats"]["used_memory"],
                    )
                elif service == "S3":
                    logger.info(
                        "S3: Region: %s, Buckets: %s",
                        details["connection"]["region"],
                        details["buckets"]["count"],
                    )
                elif service == "GITHUB":
                    logger.info(
                        "GitHub API: Rate Limit %s/%s (Reset at: %s)",
                        details["rate_limit"]["remaining"],
                        details["rate_limit"]["limit"],
                        details["rate_limit"]["reset_at"],
                    )

        # Log failed checks
        if results["error"]:
            logger.error("\nFailed Connections:")
            for result in results["error"]:
                logger.error("%s: %s", result["service"].upper(), result["error"])

            # Raise exception with failed services
            error_services = [r["service"] for r in results["error"]]
            raise AirflowException(
                f"Connection check failed for services: {', '.join(error_services)}"
            )

        # Log skipped checks
        if results["skipped"]:
            logger.info("\nSkipped Connections:")
            for result in results["skipped"]:
                logger.info("%s: %s", result["service"].upper(), result["reason"])

        return results

    except Exception as e:
        logger.error("Error checking connections: %s", str(e))
        raise


def upload_to_s3(collector: GitHubCollector, s3_client: Any, **context):
    """Task to upload data to S3"""
    try:
        # Get file paths
        repo_file, _ = get_data_file_paths(
            base_dir=BASE_DIR,
            org=collector.org_name,
            date=context["execution_date"],
            data_type="repos",
        )
        team_file, _ = get_data_file_paths(
            base_dir=BASE_DIR,
            org=collector.org_name,
            date=context["execution_date"],
            data_type="teams",
        )
        member_file, _ = get_data_file_paths(
            base_dir=BASE_DIR,
            org=collector.org_name,
            date=context["execution_date"],
            data_type="members",
        )

        # Upload files to S3
        bucket = Variable.get("S3_BUCKET", "github-data")
        for file_path in [repo_file, team_file, member_file]:
            if file_path.exists():
                s3_key = f"{collector.org_name}/{context['execution_date'].strftime('%Y/%m/%d')}/{file_path.name}"
                s3_client.upload_file(str(file_path), bucket, s3_key)
                logger.info("Uploaded %s to s3://%s/%s", file_path.name, bucket, s3_key)
            else:
                logger.warning("File not found: %s", file_path)

        return "Upload completed successfully"

    except Exception as e:
        logger.error("Error uploading data to S3: %s", str(e))
        raise


def collect_repository_data(gh_collector: GitHubCollector, **context):
    """Task to collect repository data"""
    # Get configuration
    limit = int(Variable.get("github_repo_limit", default_var="0"))
    batch_size = int(Variable.get("github_batch_size", default_var="5"))

    # Get file paths
    repo_file, latest_repo_file = get_data_file_paths(
        base_dir=BASE_DIR,
        org=gh_collector.org_name,
        date=context["execution_date"],
        data_type="repos",
    )

    async def _collect_repos():
        try:
            # Get cache stats before collection
            if gh_collector.use_redis:
                stats = gh_collector.get_cache_stats()
                logger.info("Redis cache status: %s", stats["redis_cache"]["connected"])
                logger.info(
                    "Redis cache entries: %d", stats["redis_cache"]["keys_count"]
                )

            # Get organization info
            org_data = await gh_collector.get_organization()
            logger.info("Organization: %s", org_data.get("name", gh_collector.org_name))
            logger.info(
                "Total repositories: %s",
                org_data.get("repositories", {}).get("totalCount", "unknown"),
            )

            # Get repositories
            logger.info("Fetching repositories for %s...", gh_collector.org_name)
            repos = await gh_collector.get_repositories(limit=limit)
            logger.info("Found %d repositories", len(repos))

            # Collect repository data
            logger.info("Collecting repository data with batch size %d...", batch_size)
            await gh_collector.collect_dora_metrics(
                repos=[r["name"] for r in repos],
                output_file=repo_file,
                batch_size=batch_size,
            )

            # Get cache stats after collection
            if gh_collector.use_redis:
                stats = gh_collector.get_cache_stats()
                logger.info(
                    "Final Redis cache entries: %d",
                    stats["redis_cache"]["keys_count"],
                )
                logger.info(
                    "Memory cache entries: %d", stats["memory_cache"]["entries"]
                )

            # Update symlink
            update_symlink(repo_file, latest_repo_file)
            logger.info("Data collection completed. Results saved to: %s", repo_file)
            return str(repo_file)

        except Exception as e:
            logger.error("Error collecting repository data: %s", e)
            raise

    return asyncio.run(_collect_repos())


def collect_team_data(gh_collector: GitHubCollector, **context):
    """Task to collect team data"""
    # Get file paths
    team_file, latest_team_file = get_data_file_paths(
        base_dir=BASE_DIR,
        org=gh_collector.org_name,
        date=context["execution_date"],
        data_type="teams",
    )

    async def _collect_teams():
        try:
            # Get cache stats before collection
            if gh_collector.use_redis:
                stats = gh_collector.get_cache_stats()
                logger.info("Redis cache status: %s", stats["redis_cache"]["connected"])
                logger.info(
                    "Redis cache entries: %d", stats["redis_cache"]["keys_count"]
                )

            # Get organization info
            org_data = await gh_collector.get_organization()
            logger.info("Organization: %s", org_data.get("name", gh_collector.org_name))

            # Collect team data
            logger.info("Collecting team data for %s...", gh_collector.org_name)
            teams = await gh_collector.get_teams(include_members=True)
            logger.info("Found %d teams", len(teams))

            # Save team data
            team_data = {
                "organization": gh_collector.org_name,
                "collected_at": datetime.utcnow().isoformat(),
                "teams": teams,
                "metadata": {
                    "team_count": len(teams),
                    "include_members": True,
                },
            }
            gh_collector._save_to_json(team_data, team_file)

            # Get cache stats after collection
            if gh_collector.use_redis:
                stats = gh_collector.get_cache_stats()
                logger.info(
                    "Final Redis cache entries: %d",
                    stats["redis_cache"]["keys_count"],
                )
                logger.info(
                    "Memory cache entries: %d", stats["memory_cache"]["entries"]
                )

            # Update symlink
            update_symlink(team_file, latest_team_file)
            logger.info("Team data saved to: %s", team_file)
            return str(team_file)

        except Exception as e:
            logger.error("Error collecting team data: %s", e)
            raise

    return asyncio.run(_collect_teams())


def collect_member_data(gh_collector: GitHubCollector, **context):
    """Task to collect member data"""
    # Get configuration
    batch_size = int(Variable.get("github_batch_size", default_var="10"))

    # Get file paths
    member_file, latest_member_file = get_data_file_paths(
        base_dir=BASE_DIR,
        org=gh_collector.org_name,
        date=context["execution_date"],
        data_type="members",
    )

    async def _collect_members():
        try:
            # Get cache stats before collection
            if gh_collector.use_redis:
                stats = gh_collector.get_cache_stats()
                logger.info("Redis cache status: %s", stats["redis_cache"]["connected"])
                logger.info(
                    "Redis cache entries: %d", stats["redis_cache"]["keys_count"]
                )

            logger.info(
                "Collecting member data for %s... with detailed info: True",
                gh_collector.org_name,
            )

            # Get all organization members via API
            data = await gh_collector.get_organization_members(
                include_detailed_info=True, batch_size=batch_size
            )
            logger.info(f"Found {len(data)} organization members")

            # Save data
            member_data = {
                "organization": gh_collector.org_name,
                "collected_at": datetime.utcnow().isoformat(),
                "members": data,
                "metadata": {
                    "member_count": len(data),
                    "include_detailed_info": True,
                },
            }
            gh_collector._save_to_json(member_data, member_file)

            # Get cache stats after collection
            if gh_collector.use_redis:
                stats = gh_collector.get_cache_stats()
                logger.info(
                    "Final Redis cache entries: %d",
                    stats["redis_cache"]["keys_count"],
                )
                logger.info(
                    "Memory cache entries: %d", stats["memory_cache"]["entries"]
                )

            # Update symlink
            logger.info(
                "Updating symlink for member data: %s -> %s",
                member_file,
                latest_member_file,
            )
            update_symlink(member_file, latest_member_file)
            logger.info("Member data saved to: %s", member_file)
            return str(member_file)

        except Exception as e:
            logger.error("Error collecting member data: %s", e)
            raise

    return asyncio.run(_collect_members())


def cleanup_local_files(s3_client: Any, **context):
    """Task to remove entire data/raw directory after successful S3 upload"""
    try:
        # Remove local directory
        raw_data_dir = Path("data/raw")
        if raw_data_dir.exists():
            shutil.rmtree(raw_data_dir)
            logger.info("Successfully removed directory: %s", raw_data_dir)
            return f"Removed directory: {raw_data_dir}"
        else:
            logger.info("Directory does not exist: %s", raw_data_dir)
            return "No directory to clean"

    except Exception as e:
        logger.error("Error during cleanup: %s", str(e))
        raise


# DAG Definition
with DAG(
    DAG_ID,
    default_args=default_args,
    description=DAG_DESCRIPTION,
    schedule_interval=DAG_SCHEDULE,
    start_date=DAG_START_DATE,
    catchup=False,
    tags=DAG_TAGS,
    doc_md=__doc__,
) as dag:
    # Initialize collector and S3 client
    org = Variable.get("github_org", default_var="moneyforward")
    collector = GitHubCollector.from_env(org_name=org, use_redis=True)

    # Get AWS credentials from Airflow connection
    aws_conn = BaseHook.get_connection("aws_default")
    s3_client = boto3.client(
        "s3",
        endpoint_url=Variable.get(
            "S3_ENDPOINT_URL", f"http://{aws_conn.host}:{aws_conn.port}"
        ),
        aws_access_key_id=aws_conn.login,
        aws_secret_access_key=aws_conn.password,
        region_name=Variable.get("AWS_REGION", "us-east-1"),
        verify=Variable.get("S3_SKIP_SSL_VERIFY", "false").lower() != "true",
    )

    # Check connections task
    check_connections_task = PythonOperator(
        task_id="check_connections",
        python_callable=check_connections,
        provide_context=True,
        op_kwargs={"collector": collector},
    )

    # Define collection tasks
    repo_task = PythonOperator(
        task_id="collect_repository_data",
        python_callable=collect_repository_data,
        provide_context=True,
        op_kwargs={"gh_collector": collector},
    )

    team_task = PythonOperator(
        task_id="collect_team_data",
        python_callable=collect_team_data,
        provide_context=True,
        op_kwargs={"gh_collector": collector},
    )

    member_task = PythonOperator(
        task_id="collect_member_data",
        python_callable=collect_member_data,
        provide_context=True,
        op_kwargs={"gh_collector": collector},
    )

    # Upload task with S3 client
    upload_task = PythonOperator(
        task_id="upload_to_s3",
        python_callable=upload_to_s3,
        provide_context=True,
        op_kwargs={
            "collector": collector,
            "s3_client": s3_client,
        },
    )

    # Cleanup task with S3 client for verification
    cleanup_task = PythonOperator(
        task_id="cleanup_local_files",
        python_callable=cleanup_local_files,
        provide_context=True,
        op_kwargs={"s3_client": s3_client},
    )

    # Define task dependencies
    (
        check_connections_task
        >> [repo_task, team_task, member_task]
        >> upload_task
        >> cleanup_task
    )

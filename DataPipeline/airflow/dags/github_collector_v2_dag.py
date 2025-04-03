"""
Airflow DAG for GitHub Collector V2

This DAG collects GitHub organization data in three parallel streams:
1. Repository Data: Collects detailed repository information including PRs, issues, and commits
2. Team Data: Collects team structure, members, and repository assignments
3. Member Data: Collects detailed user information and contribution metrics

The data is stored in a date-based directory structure for easy tracking and governance.
"""

from datetime import datetime, timedelta
from pathlib import Path
import json
import asyncio
from airflow import DAG
from airflow.operators.python import PythonOperator
from airflow.models import Variable
from plugins.utils.path_utils import get_data_file_paths, update_symlink
from src.github_collector_v2.collector import GitHubCollector
from src.utils.loggers import get_logger

logger = get_logger("github_collector_v2.dag")

# DAG Constants
DAG_ID = "github_collector_v2"
DAG_DESCRIPTION = (
    "Collect GitHub organization data including repositories, teams, and users"
)
DAG_SCHEDULE = timedelta(days=1)
DAG_START_DATE = datetime(2024, 1, 1)
DAG_TAGS = ["github", "data_collection"]

# Default arguments
default_args = {
    "owner": "airflow",
    "depends_on_past": False,
    "email_on_failure": False,
    "email_on_retry": False,
    "retries": 1,
    "retry_delay": timedelta(minutes=5),
}

# File path constants
BASE_DIR = Path("data/github")


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

            # Update symlink
            update_symlink(repo_file, latest_repo_file)
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
            # Collect team data
            logger.info("Fetching team information for %s...", gh_collector.org_name)
            teams = await gh_collector.get_teams(include_members=True)

            # Save team data
            with open(team_file, "w") as f:
                json.dump(
                    {
                        "teams": teams,
                        "collected_at": datetime.utcnow().isoformat(),
                        "metadata": {
                            "team_count": len(teams),
                            "include_members": True,
                        },
                    },
                    f,
                    indent=2,
                )

            # Update symlink
            update_symlink(team_file, latest_team_file)
            return str(team_file)

        except Exception as e:
            logger.error("Error collecting team data: %s", e)
            raise

    return asyncio.run(_collect_teams())


def collect_member_data(gh_collector: GitHubCollector, **context):
    """Task to collect member data"""
    # Get file paths
    member_file, latest_member_file = get_data_file_paths(
        base_dir=BASE_DIR,
        org=gh_collector.org_name,
        date=context["execution_date"],
        data_type="members",
    )

    async def _collect_members():
        try:
            # Collect member data
            logger.info(
                "Fetching member information and contributions for %s...",
                gh_collector.org_name,
            )
            members = await gh_collector.get_organization_members(
                include_detailed_info=True
            )

            # Save member data
            with open(member_file, "w") as f:
                json.dump(
                    {
                        "members": members,
                        "collected_at": datetime.utcnow().isoformat(),
                        "metadata": {
                            "member_count": len(members),
                            "include_detailed_info": True,
                        },
                    },
                    f,
                    indent=2,
                )

            # Update symlink
            update_symlink(member_file, latest_member_file)
            return str(member_file)

        except Exception as e:
            logger.error("Error collecting member data: %s", e)
            raise

    return asyncio.run(_collect_members())


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
    # Initialize collector once
    org = Variable.get("github_org", default_var="moneyforward")
    collector = GitHubCollector.from_env(org_name=org)

    # Define tasks with collector parameter
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

    # Tasks can run in parallel since they are independent
    [repo_task, team_task, member_task]

"""
GitHub Collector CLI
"""

import asyncio
import json
import click
from datetime import datetime
import os
from pathlib import Path
from dotenv import load_dotenv
from airflow.models import Variable

# Add the src directory to the path
from src.github_collector_v2.collector import GitHubCollector
from src.utils.loggers import get_logger
from src.utils.github_data_utils import (
    load_team_members_from_csv,
    extract_members_from_team_json,
    find_latest_team_member_csv,
)
from src.utils.helper_functions import ensure_output_dir

# Set up logging
logger = get_logger("github_collector_v2.cli")

OUTPUT_DIR = Variable.get("OUTPUT_DIR", "data/raw/github/")


def get_token() -> str:
    """Get GitHub token from environment"""
    token = os.getenv("GITHUB_TOKEN")
    if not token:
        raise ValueError("GITHUB_TOKEN environment variable is required")
    return token


@click.group()
def cli():
    """GitHub Collector V2 CLI"""
    load_dotenv()


@cli.command()
@click.argument("org")
@click.option("--limit", type=int, help="Limit number of repositories to collect")
@click.option(
    "--batch-size",
    type=int,
    default=5,
    help="Number of repositories to process in parallel",
)
@click.option(
    "--output-dir", type=click.Path(), default=OUTPUT_DIR, help="Output directory"
)
@click.option("--use-redis/--no-redis", default=None, help="Use Redis for caching")
@click.option("--redis-host", help="Redis host")
@click.option("--redis-port", type=int, help="Redis port")
@click.option("--redis-db", type=int, help="Redis database number")
@click.option("--clear-cache", is_flag=True, help="Clear cache before collecting data")
def collect_repository(
    org: str,
    limit: int,
    batch_size: int,
    output_dir: str,
    use_redis: bool,
    redis_host: str,
    redis_port: int,
    redis_db: int,
    clear_cache: bool,
):
    """Collect GitHub data for an organization"""

    async def _collect():
        try:
            # Prepare output directory
            output_dir_path = Path(output_dir)
            output_dir_path.mkdir(parents=True, exist_ok=True)

            # Generate output filename with timestamp
            timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
            output_file = output_dir_path / f"github_repository_{org}_{timestamp}.json"

            # Initialize collector with Redis config if specified
            kwargs = {}

            # Handle Redis configuration
            if use_redis is not None:  # Only set if explicitly specified
                kwargs["use_redis"] = use_redis
            if redis_host:
                kwargs["redis_host"] = redis_host
            if redis_port:
                kwargs["redis_port"] = redis_port
            if redis_db is not None:
                kwargs["redis_db"] = redis_db

            # Create collector
            collector = GitHubCollector.from_env(org_name=org, **kwargs)

            # Show cache configuration
            cache_enabled_type = "Redis" if collector.use_redis else "Memory"
            logger.info("Using %s cache for data collection", cache_enabled_type)

            async with collector:
                # Clear cache if requested
                if clear_cache:
                    logger.info("Clearing cache before collection")
                    collector.clear_cache()

                # Get cache stats before collection
                if collector.use_redis:
                    stats = collector.get_cache_stats()
                    logger.info(
                        "Redis cache status: %s", stats["redis_cache"]["connected"]
                    )
                    logger.info(
                        "Redis cache entries: %d", stats["redis_cache"]["keys_count"]
                    )

                # Get organization info
                org_data = await collector.get_organization()
                logger.info("Organization: %s", org_data.get("name", org))
                logger.info(
                    "Total repositories: %s",
                    org_data.get("repositories", {}).get("totalCount", "unknown"),
                )

                # Get repositories
                logger.info("Fetching repositories for %s...", org)
                repos = await collector.get_repositories(limit=limit)
                logger.info("Found %d repositories", len(repos))

                # Collect DORA metrics
                logger.info(
                    "Collecting repository data with batch size %d...", batch_size
                )
                await collector.collect_dora_metrics(
                    repos=[r["name"] for r in repos],
                    output_file=output_file,
                    batch_size=batch_size,
                )

                logger.info(
                    "Data collection completed. Results saved to: %s", output_file
                )

                # Get cache stats after collection
                if collector.use_redis:
                    stats = collector.get_cache_stats()
                    logger.info(
                        "Final Redis cache entries: %d",
                        stats["redis_cache"]["keys_count"],
                    )
                    logger.info(
                        "Memory cache entries: %d", stats["memory_cache"]["entries"]
                    )

        except Exception as e:
            logger.error("Error during collection: %s", str(e))
            raise

    asyncio.run(_collect())


@cli.command()
@click.argument("org")
@click.argument("repo")
@click.option("--use-redis/--no-redis", default=None, help="Use Redis for caching")
@click.option("--redis-host", help="Redis host")
@click.option("--redis-port", type=int, help="Redis port")
@click.option("--redis-db", type=int, help="Redis database number")
@click.option("--output", type=click.Path(), help="Save repository data to JSON file")
@click.option(
    "--force-refresh", is_flag=True, help="Force refresh from API (ignore cache)"
)
def test(
    org: str,
    repo: str,
    use_redis: bool,
    redis_host: str,
    redis_port: int,
    redis_db: int,
    output: str,
    force_refresh: bool,
):
    """Test data collection for a single repository"""

    async def _test():
        try:
            # Initialize collector with Redis config if specified
            kwargs = {}

            # Handle Redis configuration
            if use_redis is not None:  # Only set if explicitly specified
                kwargs["use_redis"] = use_redis
            if redis_host:
                kwargs["redis_host"] = redis_host
            if redis_port:
                kwargs["redis_port"] = redis_port
            if redis_db is not None:
                kwargs["redis_db"] = redis_db

            # Create collector
            collector = GitHubCollector.from_env(org_name=org, **kwargs)

            # Show cache configuration
            cache_enabled_type = "Redis" if collector.use_redis else "Memory"
            logger.info("Using %s cache for testing", cache_enabled_type)

            async with collector:
                # Show rate limit info
                rate_limit = await collector.check_rate_limit()
                logger.info(
                    "API rate limit: %s/%s",
                    rate_limit.get("remaining"),
                    rate_limit.get("limit"),
                )

                # Clear cache for this repo if force refresh is requested
                if force_refresh:
                    repo_key = f"repo_data_{repo}"
                    collector.clear_cache(prefix=repo_key)
                    logger.info("Cleared cache for repository: %s", repo)

                # Test repository data collection
                logger.info("Testing data collection for %s/%s", org, repo)
                data = await collector.get_repository_data(repo)

                # Print summary
                if "error" in data:
                    logger.error("Error: %s", data["error"])
                else:
                    repo_data = data.get("repository", {})
                    print("\nRepository Data Summary:")
                    print(
                        f"Pull Requests: {len(repo_data.get('pullRequests', {}).get('nodes', []))}"
                    )
                    print(
                        f"Issues: {len(repo_data.get('issues', {}).get('nodes', []))}"
                    )
                    print(
                        f"Deployments: {len(repo_data.get('deployments', {}).get('nodes', []))}"
                    )

                    # Show data source
                    cache_stats = collector.get_cache_stats()
                    print(f"Cache Type: {'Redis' if collector.use_redis else 'Memory'}")

                    if collector.use_redis:
                        print(
                            f"Redis Connected: {cache_stats['redis_cache']['connected']}"
                        )
                        print(f"Redis Keys: {cache_stats['redis_cache']['keys_count']}")

                # Save output if requested
                if output:
                    output_path = Path(output)
                    collector._save_to_json(data, output_path)
                    logger.info("Data saved to %s", output_path)

        except Exception as e:
            logger.error("Error during test: %s", str(e))
            raise

    asyncio.run(_test())


@cli.command()
@click.argument("org")
@click.option("--use-redis/--no-redis", default=None, help="Use Redis for caching")
@click.option("--redis-host", help="Redis host")
@click.option("--redis-port", type=int, help="Redis port")
@click.option("--redis-db", type=int, help="Redis database number")
def info(org: str, use_redis: bool, redis_host: str, redis_port: int, redis_db: int):
    """Get basic information about an organization"""

    async def _info():
        try:
            # Initialize collector with Redis config if specified
            kwargs = {}

            # Handle Redis configuration
            if use_redis is not None:  # Only set if explicitly specified
                kwargs["use_redis"] = use_redis
            if redis_host:
                kwargs["redis_host"] = redis_host
            if redis_port:
                kwargs["redis_port"] = redis_port
            if redis_db is not None:
                kwargs["redis_db"] = redis_db

            # Create collector
            collector = GitHubCollector.from_env(org_name=org, **kwargs)

            async with collector:
                # Get organization info
                org_data = await collector.get_organization()

                # Print organization info
                print("\nOrganization Information:")
                print(f"Name: {org_data.get('name')}")
                print(f"URL: {org_data.get('url')}")
                print(f"Description: {org_data.get('description')}")
                print(f"Created: {org_data.get('createdAt')}")
                print(f"Updated: {org_data.get('updatedAt')}")
                print(
                    f"Repositories: {org_data.get('repositories', {}).get('totalCount', 'unknown')}"
                )
                print(
                    f"Members: {org_data.get('membersWithRole', {}).get('totalCount', 'unknown')}"
                )

                # Get rate limit info
                rate_limit = await collector.check_rate_limit()

                print("\nAPI Rate Limit Status:")
                print(f"Limit: {rate_limit.get('limit')}")
                print(f"Remaining: {rate_limit.get('remaining')}")
                print(f"Reset At: {rate_limit.get('resetAt')}")
                print(f"Used: {rate_limit.get('used')}")

                # Print cache information
                cache_stats = collector.get_cache_stats()

                print("\nCache Information:")
                print(
                    f"Cache Type: {'Redis' if collector.use_redis else 'Memory-only'}"
                )
                print(f"Memory Cache Entries: {cache_stats['memory_cache']['entries']}")

                if collector.use_redis:
                    print("\nRedis Cache Status:")
                    print(f"Connected: {cache_stats['redis_cache']['connected']}")
                    print(f"Total Keys: {cache_stats['redis_cache']['keys_count']}")
                    print(f"Memory Used: {cache_stats['redis_cache']['memory_used']}")

        except Exception as e:
            logger.error("Error fetching organization info: %s", str(e))
            raise

    asyncio.run(_info())


@cli.command()
@click.argument("org")
@click.option("--use-redis/--no-redis", default=None, help="Use Redis for caching")
@click.option("--redis-host", help="Redis host")
@click.option("--redis-port", type=int, help="Redis port")
@click.option("--redis-db", type=int, help="Redis database number")
@click.option("--prefix", help="Clear only keys with this prefix")
def clear_cache(
    org: str,
    use_redis: bool,
    redis_host: str,
    redis_port: int,
    redis_db: int,
    prefix: str,
):
    """Clear cache data"""

    async def _clear_cache():
        try:
            # Initialize collector with Redis config if specified
            kwargs = {}

            # Handle Redis configuration
            if use_redis is not None:  # Only set if explicitly specified
                kwargs["use_redis"] = use_redis
            if redis_host:
                kwargs["redis_host"] = redis_host
            if redis_port:
                kwargs["redis_port"] = redis_port
            if redis_db is not None:
                kwargs["redis_db"] = redis_db

            # Create collector
            collector = GitHubCollector.from_env(org_name=org, **kwargs)

            # Get cache stats before clearing
            before_stats = collector.get_cache_stats()
            memory_before = before_stats["memory_cache"]["entries"]
            redis_before = (
                before_stats["redis_cache"]["keys_count"] if collector.use_redis else 0
            )

            logger.info("Memory cache entries before: %d", memory_before)
            if collector.use_redis:
                logger.info("Redis cache entries before: %d", redis_before)

            # Clear cache
            collector.clear_cache(prefix=prefix)

            # Get cache stats after clearing
            after_stats = collector.get_cache_stats()
            memory_after = after_stats["memory_cache"]["entries"]
            redis_after = (
                after_stats["redis_cache"]["keys_count"] if collector.use_redis else 0
            )

            logger.info("Memory cache entries after: %d", memory_after)
            if collector.use_redis:
                logger.info("Redis cache entries after: %d", redis_after)

            print(f"Cleared {memory_before - memory_after} memory cache entries")
            if collector.use_redis:
                print(f"Cleared {redis_before - redis_after} Redis cache entries")

        except Exception as e:
            logger.error("Error clearing cache: %s", str(e))
            raise

    asyncio.run(_clear_cache())


@cli.command()
@click.argument("org")
@click.option(
    "--output-dir", type=click.Path(), default=OUTPUT_DIR, help="Output directory"
)
@click.option("--use-redis/--no-redis", default=None, help="Use Redis for caching")
@click.option("--redis-host", help="Redis host")
@click.option("--redis-port", type=int, help="Redis port")
@click.option("--redis-db", type=int, help="Redis database number")
@click.option("--clear-cache", is_flag=True, help="Clear cache before collecting data")
@click.option(
    "--include-members", is_flag=True, default=True, help="Include members in team data"
)
def collect_teams(
    org: str,
    output_dir: str,
    use_redis: bool,
    redis_host: str,
    redis_port: int,
    redis_db: int,
    clear_cache: bool,
    include_members: bool,
):
    """Collect team data for an organization"""

    async def _collect_teams():
        try:
            # Prepare output directory
            output_dir_path = Path(output_dir)
            output_dir_path.mkdir(parents=True, exist_ok=True)

            # Generate output filename with timestamp
            timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
            team_file = output_dir_path / f"github_teams_{org}_{timestamp}.json"

            # Initialize collector with Redis config if specified
            kwargs = {}

            # Handle Redis configuration
            if use_redis is not None:  # Only set if explicitly specified
                kwargs["use_redis"] = use_redis
            if redis_host:
                kwargs["redis_host"] = redis_host
            if redis_port:
                kwargs["redis_port"] = redis_port
            if redis_db is not None:
                kwargs["redis_db"] = redis_db

            # Create collector
            collector = GitHubCollector.from_env(org_name=org, **kwargs)

            # Show cache configuration
            cache_enabled_type = "Redis" if collector.use_redis else "Memory"
            logger.info("Using %s cache for data collection", cache_enabled_type)

            async with collector:
                # Clear cache if requested
                if clear_cache:
                    logger.info("Clearing cache before collection")
                    collector.clear_cache(prefix="teams")

                # Get cache stats before collection
                if collector.use_redis:
                    stats = collector.get_cache_stats()
                    logger.info(
                        "Redis cache status: %s", stats["redis_cache"]["connected"]
                    )
                    logger.info(
                        "Redis cache entries: %d", stats["redis_cache"]["keys_count"]
                    )

                # Get organization info
                org_data = await collector.get_organization()
                logger.info("Organization: %s", org_data.get("name", org))

                # Collect team data
                logger.info("Collecting team data for %s...", org)
                teams = await collector.get_teams(include_members=include_members)
                logger.info("Found %d teams", len(teams))

                # Save team data
                team_data = {
                    "organization": org,
                    "collected_at": datetime.utcnow().isoformat(),
                    "teams": teams,
                    "metadata": {
                        "team_count": len(teams),
                        "include_members": include_members,
                    },
                }
                collector._save_to_json(team_data, team_file)
                logger.info("Team data saved to %s", team_file)

                # Print summary
                print("\nTeam Collection Summary:")
                print(f"Teams: {len(teams)}")
                print(f"Team data saved to: {team_file}")

                # Get cache stats after collection
                if collector.use_redis:
                    stats = collector.get_cache_stats()
                    logger.info(
                        "Final Redis cache entries: %d",
                        stats["redis_cache"]["keys_count"],
                    )
                    logger.info(
                        "Memory cache entries: %d", stats["memory_cache"]["entries"]
                    )

        except Exception as e:
            logger.error("Error during team collection: %s", str(e))
            raise

    asyncio.run(_collect_teams())


@cli.command()
@click.argument("org")
@click.option(
    "--output-dir", type=click.Path(), default=OUTPUT_DIR, help="Output directory"
)
@click.option("--use-redis/--no-redis", default=None, help="Use Redis for caching")
@click.option("--redis-host", help="Redis host")
@click.option("--redis-port", type=int, help="Redis port")
@click.option("--redis-db", type=int, help="Redis database number")
@click.option("--clear-cache", is_flag=True, help="Clear cache before collecting data")
@click.option(
    "--team-file",
    type=click.Path(),
    help="Path to team.json file containing team data with member logins",
)
@click.option(
    "--team-csv",
    type=click.Path(),
    help="Path to CSV file containing team member logins",
)
@click.option(
    "--batch-size",
    type=int,
    default=10,
    help="Number of members to process in parallel",
)
def collect_members_contributions(
    org: str,
    output_dir: str,
    use_redis: bool,
    redis_host: str,
    redis_port: int,
    redis_db: int,
    clear_cache: bool,
    team_file: str,
    team_csv: str,
    batch_size: int,
):
    """Collect member data for an organization"""

    async def _collect_members():
        try:
            # Prepare output directory
            output_dir_path = ensure_output_dir(output_dir)

            # Generate output filename with timestamp
            timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
            member_contributions_file = (
                output_dir_path / f"github_members_contributions_{org}_{timestamp}.json"
            )

            # Initialize collector with Redis config if specified
            kwargs = {}

            # Handle Redis configuration
            if use_redis is not None:  # Only set if explicitly specified
                kwargs["use_redis"] = use_redis
            if redis_host:
                kwargs["redis_host"] = redis_host
            if redis_port:
                kwargs["redis_port"] = redis_port
            if redis_db is not None:
                kwargs["redis_db"] = redis_db

            # Create collector
            collector = GitHubCollector.from_env(org_name=org, **kwargs)

            # Show cache configuration
            cache_enabled_type = "Redis" if collector.use_redis else "Memory"
            logger.info("Using %s cache for data collection", cache_enabled_type)

            async with collector:
                # Clear cache if requested
                if clear_cache:
                    logger.info("Clearing members cache before collection")
                    collector.clear_cache(prefix="user_contrib")

                # Determine how to get members
                members = []

                # Option 1: Extract from team file
                if team_file:
                    logger.info(f"Extracting members from team file: {team_file}")
                    members = extract_members_from_team_json(team_file)

                # Option 2: Extract from CSV file
                elif team_csv:
                    logger.info(f"Loading members from CSV file: {team_csv}")
                    # Check if the file exists
                    if not Path(team_csv).exists():
                        raise FileNotFoundError(f"CSV file does not exist: {team_csv}")
                    members = load_team_members_from_csv(team_csv)

                # Option 3: Try to find latest team CSV if available
                elif not members:
                    # Try to find latest team members CSV file
                    latest_csv = find_latest_team_member_csv(output_dir, org)
                    if latest_csv:
                        logger.info(f"Found latest team members CSV: {latest_csv}")
                        members = load_team_members_from_csv(latest_csv)

                # Option 4: Get all organization members via API
                if not members:
                    raise ValueError("No members found")

                # Log the members we found
                logger.info(f"Collecting data for {len(members)} members")

                # Collect member data with contributions if detailed_info is True
                logger.info(
                    "Collecting detailed member information and contributions..."
                )
                data = await collector.collect_user_contributions(
                    members, batch_size=batch_size, detailed_info=True
                )

                # Save data
                with open(member_contributions_file, "w") as f:
                    json.dump(
                        {
                            "organization": org,
                            "collected_at": datetime.utcnow().isoformat(),
                            "members_contributions": data,
                            "metadata": {
                                "member_count": len(data),
                            },
                        },
                        f,
                        indent=2,
                    )

                logger.info(
                    "Member contributions data saved to: %s", member_contributions_file
                )
                print(
                    f"\nMember contributions data saved to: {member_contributions_file}"
                )

        except Exception as e:
            logger.error("Error collecting member data: %s", str(e))
            raise

    asyncio.run(_collect_members())


@cli.command()
@click.argument("org")
@click.option(
    "--output-dir", type=click.Path(), default=OUTPUT_DIR, help="Output directory"
)
@click.option("--use-redis/--no-redis", default=None, help="Use Redis for caching")
@click.option("--redis-host", help="Redis host")
@click.option("--redis-port", type=int, help="Redis port")
@click.option("--redis-db", type=int, help="Redis database number")
@click.option("--clear-cache", is_flag=True, help="Clear cache before collecting data")
@click.option(
    "--detailed-info",
    is_flag=True,
    default=True,
    help="Include detailed member information",
)
@click.option(
    "--batch-size",
    type=int,
    default=10,
    help="Number of members to process in parallel",
)
def collect_members(
    org: str,
    output_dir: str,
    use_redis: bool,
    redis_host: str,
    redis_port: int,
    redis_db: int,
    clear_cache: bool,
    detailed_info: bool,
    batch_size: int,
):
    """Collect member data for an organization"""

    async def _collect_members():
        try:
            # Prepare output directory
            output_dir_path = ensure_output_dir(output_dir)

            # Generate output filename with timestamp
            timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
            member_file = output_dir_path / f"github_members_{org}_{timestamp}.json"

            # Initialize collector with Redis config if specified
            kwargs = {}

            # Handle Redis configuration
            if use_redis is not None:  # Only set if explicitly specified
                kwargs["use_redis"] = use_redis
            if redis_host:
                kwargs["redis_host"] = redis_host
            if redis_port:
                kwargs["redis_port"] = redis_port
            if redis_db is not None:
                kwargs["redis_db"] = redis_db

            # Create collector
            collector = GitHubCollector.from_env(org_name=org, **kwargs)

            # Show cache configuration
            cache_enabled_type = "Redis" if collector.use_redis else "Memory"
            logger.info("Using %s cache for data collection", cache_enabled_type)

            async with collector:
                # Clear cache if requested
                if clear_cache:
                    logger.info("Clearing members cache before collection")
                    collector.clear_cache(prefix="user_contrib")
                logger.info(
                    "Collecting member data for %s... with detailed info: %s",
                    org,
                    detailed_info,
                )
                # Get all organization members via API
                data = await collector.get_organization_members(
                    include_detailed_info=detailed_info, batch_size=batch_size
                )
                logger.info(f"Found {len(data)} organization members")

                # Log the members we found
                logger.info(f"Collecting data for {len(data)} members")

                # Save data
                with open(member_file, "w") as f:
                    json.dump(
                        {
                            "organization": org,
                            "collected_at": datetime.utcnow().isoformat(),
                            "members": data,
                            "metadata": {
                                "member_count": len(data),
                                "include_detailed_info": detailed_info,
                            },
                        },
                        f,
                        indent=2,
                    )

                logger.info("Member data saved to: %s", member_file)
                print(
                    f"\nMember Collection Summary:\nMembers: {len(data)}\nDetailed info: {detailed_info}\nSaved to: {member_file}"
                )

        except Exception as e:
            logger.error("Error collecting member data: %s", str(e))
            raise

    asyncio.run(_collect_members())


def main():
    """CLI entry point"""
    cli()


if __name__ == "__main__":
    main()

"""
GitHub Collector CLI
"""

import asyncio
import click
from datetime import datetime
import os
from pathlib import Path
from dotenv import load_dotenv
from src.github_collector_v2.collector import GitHubCollector
from src.utils.loggers import get_logger

# Set up logging
logger = get_logger("github_collector_v2.cli")


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
    "--output-dir", type=click.Path(), default="data/github", help="Output directory"
)
@click.option("--use-redis/--no-redis", default=None, help="Use Redis for caching")
@click.option("--redis-host", help="Redis host")
@click.option("--redis-port", type=int, help="Redis port")
@click.option("--redis-db", type=int, help="Redis database number")
@click.option("--clear-cache", is_flag=True, help="Clear cache before collecting data")
def collect(
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
            output_file = output_dir_path / f"github_data_{org}_{timestamp}.json"

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


def main():
    """CLI entry point"""
    cli()


if __name__ == "__main__":
    main()

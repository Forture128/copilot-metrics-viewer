#!/usr/bin/env python3
"""
Example script demonstrating the GitHub collector with Redis caching
"""

import asyncio
import json
import logging
import os
import sys
from pathlib import Path

# Add the parent directory to the path so we can import the module
sys.path.append(str(Path(__file__).parents[3]))

from src.github_collector_v2.collector import GitHubCollector

# Set up logging
logging.basicConfig(
    level=logging.INFO, format="%(asctime)s - %(name)s - %(levelname)s - %(message)s"
)
logger = logging.getLogger(__name__)


async def demonstrate_redis_caching():
    """
    Demonstrates the GitHub collector with Redis caching enabled
    """
    # Get GitHub token from environment or fail
    token = os.environ.get("GITHUB_TOKEN")
    if not token:
        logger.error("GITHUB_TOKEN environment variable not set")
        sys.exit(1)

    org_name = "microsoft"  # Example organization
    repo_name = "vscode"  # Example repository

    # First run: Will populate the cache
    logger.info("First run: Fetching data from GitHub API and populating cache")
    collector = GitHubCollector(
        token=token,
        org_name=org_name,
        use_redis=True,
        redis_host="localhost",
        redis_port=6379,
        redis_db=0,
        redis_password=None,
        cache_ttl=3600,
    )

    async with collector:
        # Display cache stats before collection
        logger.info("Cache stats before collection:")
        stats = collector.get_cache_stats()
        logger.info(f"Memory cache: {stats['memory_cache']['entries']} entries")
        if collector.use_redis:
            logger.info(f"Redis cache: {stats['redis_cache']['keys_count']} keys")

        # Get organization information
        logger.info(f"Fetching organization info for {org_name}")
        org_data = await collector.get_organization()
        logger.info(f"Organization: {org_data.get('name')} ({org_data.get('login')})")

        # Get repository information
        logger.info(f"Fetching repository info for {repo_name}")
        repo_data = await collector.get_repository_data(repo_name)
        logger.info(
            f"Repository: {repo_data.get('name')} - {repo_data.get('description')[:50]}..."
        )

        # Display cache stats after collection
        logger.info("Cache stats after collection:")
        stats = collector.get_cache_stats()
        logger.info(f"Memory cache: {stats['memory_cache']['entries']} entries")
        if collector.use_redis:
            logger.info(f"Redis cache: {stats['redis_cache']['keys_count']} keys")

    # Second run: Should use cached data
    logger.info("\nSecond run: Should use cached data from Redis")
    collector2 = GitHubCollector(
        token=token,
        org_name=org_name,
        use_redis=True,
        redis_host="localhost",
        redis_port=6379,
        redis_db=0,
        redis_password=None,
        cache_ttl=3600,
    )

    async with collector2:
        # Display cache stats before collection
        logger.info("Cache stats before collection:")
        stats = collector2.get_cache_stats()
        logger.info(f"Memory cache: {stats['memory_cache']['entries']} entries")
        if collector2.use_redis:
            logger.info(f"Redis cache: {stats['redis_cache']['keys_count']} keys")

        # Get the same data again (should come from cache)
        logger.info(f"Fetching organization info for {org_name} (from cache)")
        start_time = asyncio.get_event_loop().time()
        org_data = await collector2.get_organization()
        end_time = asyncio.get_event_loop().time()
        logger.info(f"Time taken: {end_time - start_time:.4f} seconds")

        logger.info(f"Fetching repository info for {repo_name} (from cache)")
        start_time = asyncio.get_event_loop().time()
        repo_data = await collector2.get_repository_data(repo_name)
        end_time = asyncio.get_event_loop().time()
        logger.info(f"Time taken: {end_time - start_time:.4f} seconds")

        # Display cache hits and misses
        logger.info("Cache stats:")
        stats = collector2.get_cache_stats()
        logger.info(f"Memory cache hits: {stats['memory_cache'].get('hits', 0)}")
        logger.info(f"Memory cache misses: {stats['memory_cache'].get('misses', 0)}")
        if collector2.use_redis:
            logger.info(f"Redis cache hits: {stats['redis_cache'].get('hits', 0)}")
            logger.info(f"Redis cache misses: {stats['redis_cache'].get('misses', 0)}")

    # Third run: With force refresh
    logger.info("\nThird run: With force refresh (bypassing cache)")
    collector3 = GitHubCollector(
        token=token,
        org_name=org_name,
        use_redis=True,
        redis_host="localhost",
        redis_port=6379,
        redis_db=0,
        redis_password=None,
        cache_ttl=3600,
    )

    async with collector3:
        # Get the same data with force refresh
        logger.info(f"Force refreshing organization info for {org_name}")
        org_data = await collector3.get_organization(force_refresh=True)

        # Clear the cache and check stats
        logger.info("Clearing cache")
        await collector3.clear_cache()

        stats = collector3.get_cache_stats()
        logger.info(
            f"Memory cache entries after clearing: {stats['memory_cache']['entries']}"
        )
        if collector3.use_redis:
            logger.info(
                f"Redis cache keys after clearing: {stats['redis_cache']['keys_count']}"
            )


async def main():
    """Main function"""
    await demonstrate_redis_caching()


if __name__ == "__main__":
    asyncio.run(main())

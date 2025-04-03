"""
GitHub Data Collector - A comprehensive data collection library for GitHub.

This package provides tools for collecting GitHub data using both REST API
(via PyGithub) and GraphQL API. It includes features like concurrency,
rate limiting, fault tolerance, and caching.
"""

from src.github_collector.collector import (
    GitHubCollector,
    CacheManager,
    RateLimitStatus,
    retry_on_exception,
    rate_limit_aware,
    DEFAULT_RETRY_COUNT,
    DEFAULT_RETRY_BACKOFF,
    DEFAULT_CONCURRENT_REQUESTS,
    DEFAULT_RATE_LIMIT_BUFFER,
    DEFAULT_CACHE_DIR,
    DEFAULT_CACHE_TTL,
)

from src.github_collector.extended_collector import (
    ExtendedGitHubCollector,
)

from src.github_collector.graphql_collector import (
    GitHubGraphQLCollector,
)

__all__ = [
    "GitHubCollector",
    "ExtendedGitHubCollector",
    "GitHubGraphQLCollector",
    "CacheManager",
    "RateLimitStatus",
    "retry_on_exception",
    "rate_limit_aware",
    "DEFAULT_RETRY_COUNT",
    "DEFAULT_RETRY_BACKOFF",
    "DEFAULT_CONCURRENT_REQUESTS",
    "DEFAULT_RATE_LIMIT_BUFFER",
    "DEFAULT_CACHE_DIR",
    "DEFAULT_CACHE_TTL",
]

__version__ = "1.0.0"

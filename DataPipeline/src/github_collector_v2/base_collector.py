"""
Base GitHub Collector with core functionality and shared utilities
"""

from typing import Optional, Dict, List, Any, TypeVar, Generic, Callable
import asyncio
from datetime import datetime, timedelta
import aiohttp
import json
from pathlib import Path
import logging
import random
import redis

# Set up basic logging - will be replaced by your proper logger
logging.basicConfig(
    level=logging.INFO, format="%(asctime)s - %(name)s - %(levelname)s - %(message)s"
)
logger = logging.getLogger("github_collector_v2.base_collector")

# Type variables for caching
T = TypeVar("T")

# Constants
DEFAULT_CACHE_TTL = 3600  # 1 hour
DEFAULT_RATE_LIMIT_BUFFER = 100
DEFAULT_RETRY_COUNT = 3
DEFAULT_RETRY_BACKOFF = 2.0
DEFAULT_MAX_BATCH_SIZE = 5


class GraphQLError(Exception):
    """Custom exception for GraphQL errors"""

    def __init__(self, message: str, errors: Optional[List[Dict[str, Any]]] = None):
        super().__init__(message)
        self.errors = errors or []

    def __str__(self) -> str:
        if self.errors:
            return f"{super().__str__()} - Errors: {self.errors}"
        return super().__str__()


class RateLimitExceededError(Exception):
    """Exception raised when rate limit is exceeded"""

    def __init__(self, reset_at: datetime):
        self.reset_at = reset_at
        wait_seconds = (reset_at - datetime.now()).total_seconds()
        super().__init__(
            f"Rate limit exceeded. Reset at {reset_at} (in {wait_seconds:.2f} seconds)"
        )


class CacheEntry(Generic[T]):
    """A cached entry with expiration"""

    def __init__(self, data: T, ttl: int = DEFAULT_CACHE_TTL):
        self.data = data
        self.expiry = datetime.now() + timedelta(seconds=ttl)

    def is_valid(self) -> bool:
        """Check if the cache entry is still valid"""
        return datetime.now() < self.expiry


class BaseGitHubCollector:
    """Base GitHub collector with core functionality"""

    def __init__(
        self,
        token: str,
        org_name: str,
        cache_ttl: int = DEFAULT_CACHE_TTL,
        rate_limit_buffer: int = DEFAULT_RATE_LIMIT_BUFFER,
        cache_dir: Optional[Path] = None,
        log_dir: Optional[Path] = None,
        max_retries: int = DEFAULT_RETRY_COUNT,
        use_redis: bool = False,
        redis_host: str = "localhost",
        redis_port: int = 6379,
        redis_db: int = 0,
    ):
        """
        Initialize the GitHub collector

        Args:
            token: GitHub API token
            org_name: GitHub organization name
            cache_ttl: Time-to-live for cache entries in seconds
            rate_limit_buffer: Number of requests to keep in reserve before waiting
            cache_dir: Directory to store disk cache files
            log_dir: Directory to store logs
            max_retries: Maximum number of retries for failed requests
            use_redis: Whether to use Redis for caching
            redis_host: Redis host
            redis_port: Redis port
            redis_db: Redis database number
        """
        self.token = token
        self.org_name = org_name

        # Settings
        self.cache_ttl = cache_ttl
        self.rate_limit_buffer = rate_limit_buffer
        self.max_retries = max_retries

        # Cache configurations
        self.cache_dir = cache_dir or Path("data/github_cache")
        self.cache_dir.mkdir(parents=True, exist_ok=True)

        # Memory caches
        self._memory_cache: Dict[str, CacheEntry[Any]] = {}
        self._rate_limit_info: Optional[Dict[str, Any]] = None
        self._org_cache: Optional[Dict[str, Any]] = None

        # Session state
        self._session: Optional[aiohttp.ClientSession] = None

        # Set up Redis cache if enabled and available
        self.use_redis = use_redis
        self._redis_client = None

        # Flag to avoid recursion in rate limit checking
        self._checking_rate_limit = False

        if self.use_redis:
            try:
                self._redis_client = redis.Redis(
                    host=redis_host,
                    port=redis_port,
                    db=redis_db,
                    socket_timeout=5,
                    decode_responses=False,  # Keep as bytes for proper JSON handling
                )
                # Test connection
                self._redis_client.ping()
                logger.info(
                    f"Redis cache enabled: {redis_host}:{redis_port}, db={redis_db}"
                )
            except Exception as e:
                logger.warning(
                    f"Failed to connect to Redis: {str(e)}. Redis caching disabled."
                )
                self.use_redis = False
                self._redis_client = None

        logger.info(f"Collector initialized for organization: {org_name}")

    async def _get_session(self) -> aiohttp.ClientSession:
        """Get or create an aiohttp session"""
        if self._session is None or self._session.closed:
            headers = {
                "Authorization": f"Bearer {self.token}",
                "Accept": "application/vnd.github.v3+json",
                "Content-Type": "application/json",
            }
            self._session = aiohttp.ClientSession(headers=headers)
            logger.debug("Created new aiohttp session")
        return self._session

    def _cache_key(self, key_parts: List[Any]) -> str:
        """Generate a cache key from parts"""
        # Create a string representation of the key parts
        key = "-".join(str(part) for part in key_parts if part is not None)
        return f"{self.org_name}_{key}"

    def _get_from_memory_cache(self, key: str) -> Optional[Any]:
        """Get data from memory cache if available and not expired"""
        if key in self._memory_cache:
            entry = self._memory_cache[key]
            if entry.is_valid():
                logger.debug(f"Memory cache hit for key: {key}")
                return entry.data
            else:
                # Remove expired entry
                logger.debug(f"Removing expired memory cache entry for key: {key}")
                del self._memory_cache[key]
        return None

    def _set_memory_cache(self, key: str, data: Any) -> None:
        """Store data in memory cache with expiration"""
        self._memory_cache[key] = CacheEntry(data, self.cache_ttl)
        logger.debug(f"Stored data in memory cache for key: {key}")

    def _get_from_redis_cache(self, key: str) -> Optional[Any]:
        """Get data from Redis cache if available"""
        if not self.use_redis or not self._redis_client:
            return None

        try:
            redis_key = f"github:{key}"
            data = self._redis_client.get(redis_key)
            if data:
                logger.debug(f"Redis cache hit for key: {key}")
                return json.loads(data)
        except Exception as e:
            logger.warning(f"Error getting key {key} from Redis: {str(e)}")

        return None

    def _set_redis_cache(self, key: str, data: Any) -> bool:
        """Store data in Redis cache with expiration"""
        if not self.use_redis or not self._redis_client:
            return False

        try:
            redis_key = f"github:{key}"
            serialized = json.dumps(data)
            return bool(self._redis_client.setex(redis_key, self.cache_ttl, serialized))
        except Exception as e:
            logger.warning(f"Error setting key {key} in Redis: {str(e)}")
            return False

    def _get_from_cache(self, key: str) -> Optional[Any]:
        """Get data from any available cache (Redis first, then memory)"""
        # Try Redis first if enabled
        if self.use_redis:
            redis_data = self._get_from_redis_cache(key)
            if redis_data is not None:
                # Also update memory cache
                self._set_memory_cache(key, redis_data)
                return redis_data

        # Fall back to memory cache
        return self._get_from_memory_cache(key)

    def _set_cache(self, key: str, data: Any) -> None:
        """Store data in all available caches"""
        # Store in memory cache
        self._set_memory_cache(key, data)

        # Store in Redis if enabled
        if self.use_redis:
            self._set_redis_cache(key, data)

    def clear_cache(self, prefix: Optional[str] = None) -> None:
        """
        Clear cache entries

        Args:
            prefix: Optional prefix to match cache keys to invalidate
        """
        # Clear memory cache
        if prefix:
            keys_to_remove = [
                k for k in self._memory_cache.keys() if k.startswith(prefix)
            ]
            for key in keys_to_remove:
                del self._memory_cache[key]
            logger.debug(
                f"Cleared {len(keys_to_remove)} memory cache entries with prefix '{prefix}'"
            )
        else:
            count = len(self._memory_cache)
            self._memory_cache.clear()
            logger.debug(f"Cleared all {count} memory cache entries")

        # Clear Redis cache if enabled
        if self.use_redis and self._redis_client:
            try:
                pattern = f"github:{prefix or ''}*"
                keys = list(self._redis_client.scan_iter(match=pattern))
                if keys:
                    deleted = self._redis_client.delete(*keys)
                    logger.debug(f"Cleared {deleted} Redis cache entries")
            except Exception as e:
                logger.warning(f"Error clearing Redis cache: {str(e)}")

    async def _should_wait_for_rate_limit(self) -> bool:
        """Check if we should wait for rate limit to reset"""
        if self._rate_limit_info is None:
            # If we don't have rate limit info, get it
            if not self._checking_rate_limit:
                await self.check_rate_limit()

        if self._rate_limit_info:
            remaining = self._rate_limit_info.get("remaining", 1000)
            if remaining <= self.rate_limit_buffer:
                return True

        return False

    async def _wait_for_rate_limit_reset(self) -> None:
        """Wait until rate limit resets"""
        if self._rate_limit_info:
            reset_at = datetime.fromisoformat(
                self._rate_limit_info.get("resetAt", "").replace("Z", "+00:00")
            )
            now = datetime.now(reset_at.tzinfo)

            if reset_at > now:
                wait_seconds = (
                    reset_at - now
                ).total_seconds() + 5  # Add 5 seconds buffer
                logger.warning(
                    f"Rate limit nearly reached. Waiting {wait_seconds:.2f} seconds until reset."
                )
                await asyncio.sleep(wait_seconds)
                # Refresh rate limit info after waiting
                await self.check_rate_limit()

    async def execute_graphql(
        self,
        query: str,
        variables: Optional[Dict[str, Any]] = None,
        use_cache: bool = True,
    ) -> Dict[str, Any]:
        """
        Execute a GraphQL query with automatic error handling and caching

        Args:
            query: GraphQL query string
            variables: Query variables
            use_cache: Whether to use caching

        Returns:
            Query result data
        """
        variables = variables or {}

        # Generate cache key if caching is enabled
        cache_key = None
        if use_cache:
            # Create a cache key from the query and variables
            cache_key = self._cache_key(
                ["graphql", hash(query), json.dumps(variables, sort_keys=True)]
            )
            # Check cache
            cached_data = self._get_from_cache(cache_key)
            if cached_data:
                return cached_data

        # Check if we need to wait for rate limit
        if await self._should_wait_for_rate_limit():
            await self._wait_for_rate_limit_reset()

        # Clean whitespace and format query
        clean_query = " ".join(
            line.strip() for line in query.split("\n") if line.strip()
        )

        # Prepare GraphQL request
        payload = {"query": clean_query, "variables": variables}

        # Execute with retries
        retry_count = 0
        last_error = None

        while retry_count <= self.max_retries:
            try:
                session = await self._get_session()
                logger.debug(
                    f"Executing GraphQL query (attempt {retry_count + 1}/{self.max_retries + 1})"
                )

                start_time = datetime.now()
                async with session.post(
                    "https://api.github.com/graphql",
                    json=payload,
                    raise_for_status=True,
                ) as response:
                    duration = (datetime.now() - start_time).total_seconds()
                    result = await response.json()
                    logger.debug(f"GraphQL query completed in {duration:.2f}s")

                    # Check for rate limit in headers if available
                    if "X-RateLimit-Remaining" in response.headers:
                        remaining = int(
                            response.headers.get("X-RateLimit-Remaining", "1000")
                        )
                        if remaining <= self.rate_limit_buffer:
                            logger.warning(
                                f"Rate limit warning: {remaining} requests remaining"
                            )

                    # Check for errors in the GraphQL response
                    if "errors" in result:
                        error_messages = [
                            error.get("message", "Unknown error")
                            for error in result.get("errors", [])
                        ]
                        # Check if it's a rate limit error
                        for error in result.get("errors", []):
                            if (
                                "rate limit exceeded"
                                in error.get("message", "").lower()
                            ):
                                reset_at_str = response.headers.get("X-RateLimit-Reset")
                                if reset_at_str:
                                    reset_at = datetime.fromtimestamp(int(reset_at_str))
                                    raise RateLimitExceededError(reset_at)

                        logger.error(f"GraphQL errors: {error_messages}")
                        raise GraphQLError(
                            "GraphQL query returned errors", result.get("errors")
                        )

                    data = result.get("data", {})

                    # Cache successful result if caching is enabled
                    if use_cache and cache_key:
                        self._set_cache(cache_key, data)

                    return data

            except RateLimitExceededError as e:
                logger.warning(f"Rate limit exceeded: {str(e)}")
                await self._wait_for_rate_limit_reset()
                retry_count += 1

            except GraphQLError:
                # Don't retry on GraphQL errors (they're usually not transient)
                raise

            except Exception as e:
                logger.error(
                    f"Error executing GraphQL query (attempt {retry_count + 1}/{self.max_retries + 1}): {str(e)}"
                )
                last_error = e
                retry_count += 1

                if retry_count > self.max_retries:
                    break

                # Exponential backoff
                sleep_time = (DEFAULT_RETRY_BACKOFF**retry_count) + random.uniform(0, 1)
                logger.info(f"Retrying in {sleep_time:.2f} seconds...")
                await asyncio.sleep(sleep_time)

        error_msg = f"Failed to execute GraphQL query after {self.max_retries + 1} attempts. Last error: {str(last_error)}"
        logger.error(error_msg)
        raise Exception(error_msg)

    async def get_organization(self, force_refresh: bool = False) -> Dict[str, Any]:
        """
        Get and cache organization data

        Args:
            force_refresh: Force refresh from API even if cached

        Returns:
            Organization data
        """
        # Return from memory if available and not forcing refresh
        if self._org_cache is not None and not force_refresh:
            return self._org_cache

        # Try to get from cache if not forcing refresh
        if not force_refresh:
            cache_key = self._cache_key(["organization"])
            cached_data = self._get_from_cache(cache_key)
            if cached_data:
                self._org_cache = cached_data
                return cached_data

        # Fetch from API
        logger.info(f"Fetching organization data for: {self.org_name}")
        query = """
            query($org: String!) {
                organization(login: $org) {
                    id
                    name
                    url
                    description
                    createdAt
                    updatedAt
                    repositories {
                        totalCount
                    }
                    membersWithRole {
                        totalCount
                    }
                }
            }
        """
        variables = {"org": self.org_name}
        result = await self.execute_graphql(query, variables, use_cache=False)
        org_data = result.get("organization", {})

        # Cache the result
        if org_data:
            self._org_cache = org_data
            cache_key = self._cache_key(["organization"])
            self._set_cache(cache_key, org_data)

        return org_data

    async def check_rate_limit(self) -> Dict[str, Any]:
        """
        Check current rate limit status

        Returns:
            Rate limit information
        """
        # Avoid recursion
        if self._checking_rate_limit:
            # Return current rate limit info or a default if not available
            return self._rate_limit_info or {
                "limit": 5000,
                "remaining": 5000,
                "resetAt": "",
            }

        logger.debug("Checking rate limit status")

        try:
            self._checking_rate_limit = True
            query = """
                query {
                    rateLimit {
                        limit
                        remaining
                        resetAt
                        used
                    }
                }
            """
            result = await self.execute_graphql(query, use_cache=False)
            rate_limit = result.get("rateLimit", {})
            self._rate_limit_info = rate_limit

            logger.info(
                f"Rate limit status - Remaining: {rate_limit.get('remaining')}/{rate_limit.get('limit')}, "
                f"Reset at: {rate_limit.get('resetAt')}"
            )
            return rate_limit
        finally:
            self._checking_rate_limit = False

    async def batch_process(
        self,
        items: List[Any],
        process_func: Callable[[Any], Any],
        batch_size: int = DEFAULT_MAX_BATCH_SIZE,
    ) -> List[Any]:
        """
        Process a list of items in batches

        Args:
            items: List of items to process
            process_func: Async function to process each item
            batch_size: Number of items to process in parallel

        Returns:
            List of processed results
        """
        results = []

        for i in range(0, len(items), batch_size):
            batch = items[i : i + batch_size]
            logger.info(
                f"Processing batch {i // batch_size + 1} of {(len(items) + batch_size - 1) // batch_size}"
            )

            # Create tasks for each item in the batch
            tasks = [process_func(item) for item in batch]

            # Process the batch
            batch_results = await asyncio.gather(*tasks, return_exceptions=True)
            results.extend(batch_results)

            # Check for rate limit after each batch
            rate_limit = await self.check_rate_limit()
            if int(rate_limit.get("remaining", 1000)) <= self.rate_limit_buffer:
                await self._wait_for_rate_limit_reset()

        return results

    def get_cache_stats(self) -> Dict[str, Any]:
        """
        Get cache statistics

        Returns:
            Dictionary with cache statistics
        """
        stats = {
            "memory_cache": {
                "entries": len(self._memory_cache),
                "has_org_data": self._org_cache is not None,
                "has_rate_limit_info": self._rate_limit_info is not None,
            },
            "redis_cache": {
                "enabled": self.use_redis,
                "connected": False,
                "keys_count": 0,
                "memory_used": "N/A",
            },
        }

        # Get Redis stats if enabled
        if self.use_redis and self._redis_client:
            try:
                # Check connection
                self._redis_client.ping()
                stats["redis_cache"]["connected"] = True

                # Get keys count
                pattern = f"github:{self.org_name}_*"
                keys_count = len(list(self._redis_client.scan_iter(match=pattern)))
                stats["redis_cache"]["keys_count"] = keys_count

                # Get memory info
                info = self._redis_client.info()
                stats["redis_cache"]["memory_used"] = info.get(
                    "used_memory_human", "N/A"
                )

            except Exception as e:
                logger.warning(f"Error getting Redis stats: {str(e)}")

        return stats

    async def close(self) -> None:
        """Clean up all resources"""
        logger.info("Closing collector resources")
        if (
            hasattr(self, "_session")
            and self._session is not None
            and not self._session.closed
        ):
            await self._session.close()
            logger.debug("Closed aiohttp session")
        logger.info("All collector resources closed")

    async def __aenter__(self) -> "BaseGitHubCollector":
        """Support for async context manager"""
        return self

    async def __aexit__(self, exc_type, exc_val, exc_tb) -> None:
        """Cleanup resources on exit"""
        await self.close()

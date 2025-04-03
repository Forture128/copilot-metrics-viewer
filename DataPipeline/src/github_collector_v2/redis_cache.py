"""
Redis-based cache implementation for GitHub Collector

This module provides Redis caching for GitHub data with TTL support
and serialization/deserialization of complex objects.
"""

import json
import logging
from typing import Any, Dict, List, Optional, TypeVar
import redis
from redis.exceptions import RedisError

# Type variables
T = TypeVar("T")

# Set up logging
logger = logging.getLogger("github_collector_v2.redis_cache")


class RedisCache:
    """Redis cache implementation for GitHub Collector with TTL support"""

    def __init__(
        self,
        host: str = "localhost",
        port: int = 6379,
        db: int = 0,
        password: Optional[str] = None,
        prefix: str = "github:",
        default_ttl: int = 3600,
        socket_timeout: int = 5,
    ):
        """
        Initialize Redis cache

        Args:
            host: Redis host
            port: Redis port
            db: Redis database number
            password: Redis password
            prefix: Key prefix for all cache entries
            default_ttl: Default time-to-live in seconds
            socket_timeout: Socket timeout in seconds
        """
        self.prefix = prefix
        self.default_ttl = default_ttl
        self._redis = redis.Redis(
            host=host,
            port=port,
            db=db,
            password=password,
            socket_timeout=socket_timeout,
            decode_responses=False,  # Keep as bytes for proper JSON handling
        )
        logger.info(f"Redis cache initialized: {host}:{port}, db={db}")
        self._test_connection()

    def _test_connection(self) -> None:
        """Test Redis connection and log the result"""
        try:
            self._redis.ping()
            logger.info("Successfully connected to Redis")
        except RedisError as e:
            logger.warning(
                f"Redis connection failed: {str(e)}. Caching will be disabled."
            )

    def _get_prefixed_key(self, key: str) -> str:
        """Add prefix to cache key"""
        return f"{self.prefix}{key}"

    def get(self, key: str) -> Optional[Any]:
        """
        Get value from cache

        Args:
            key: Cache key

        Returns:
            Cached value or None if not found or expired
        """
        prefixed_key = self._get_prefixed_key(key)
        try:
            data = self._redis.get(prefixed_key)
            if data:
                return json.loads(data)
            return None
        except (RedisError, json.JSONDecodeError) as e:
            logger.warning(f"Error getting key {key} from Redis: {str(e)}")
            return None

    def set(self, key: str, value: Any, ttl: Optional[int] = None) -> bool:
        """
        Set value in cache with TTL

        Args:
            key: Cache key
            value: Value to cache
            ttl: Time-to-live in seconds, defaults to self.default_ttl

        Returns:
            True if successful, False otherwise
        """
        prefixed_key = self._get_prefixed_key(key)
        ttl_seconds = ttl if ttl is not None else self.default_ttl

        try:
            serialized = json.dumps(value)
            return bool(self._redis.setex(prefixed_key, ttl_seconds, serialized))
        except (RedisError, TypeError) as e:
            logger.warning(f"Error setting key {key} in Redis: {str(e)}")
            return False

    def delete(self, key: str) -> bool:
        """
        Delete value from cache

        Args:
            key: Cache key

        Returns:
            True if successful, False otherwise
        """
        prefixed_key = self._get_prefixed_key(key)
        try:
            return bool(self._redis.delete(prefixed_key))
        except RedisError as e:
            logger.warning(f"Error deleting key {key} from Redis: {str(e)}")
            return False

    def clear_all(self, pattern: str = "*") -> int:
        """
        Clear all keys matching pattern

        Args:
            pattern: Key pattern to match for deletion

        Returns:
            Number of keys deleted
        """
        pattern = self._get_prefixed_key(pattern)
        try:
            pipeline = self._redis.pipeline()
            deleted_count = 0

            # Get all keys matching pattern
            for key in self._redis.scan_iter(match=pattern):
                pipeline.delete(key)
                deleted_count += 1

            # Execute deletion
            pipeline.execute()
            logger.info(f"Cleared {deleted_count} keys matching {pattern}")
            return deleted_count
        except RedisError as e:
            logger.warning(f"Error clearing Redis cache: {str(e)}")
            return 0

    def get_stats(self) -> Dict[str, Any]:
        """
        Get cache statistics

        Returns:
            Dictionary with cache statistics
        """
        stats = {
            "connected": False,
            "keys_count": 0,
            "memory_used": "0 B",
            "redis_version": "Unknown",
        }

        try:
            # Check connection
            self._redis.ping()
            stats["connected"] = True

            # Get count of keys with our prefix
            pattern = self._get_prefixed_key("*")
            stats["keys_count"] = len(list(self._redis.scan_iter(match=pattern)))

            # Get Redis info
            info = self._redis.info()
            stats["memory_used"] = f"{info.get('used_memory_human', '0 B')}"
            stats["redis_version"] = info.get("redis_version", "Unknown")

            return stats
        except RedisError as e:
            logger.warning(f"Error getting Redis stats: {str(e)}")
            return stats


class GitHubRedisCache:
    """GitHub-specific Redis cache implementation with organization context"""

    def __init__(
        self,
        org_name: str,
        redis_config: Optional[Dict[str, Any]] = None,
    ):
        """
        Initialize GitHub Redis cache

        Args:
            org_name: GitHub organization name used as part of the cache key
            redis_config: Redis configuration dictionary
        """
        self.org_name = org_name
        self.redis_config = redis_config or {}

        # Set default prefix to include organization
        if "prefix" not in self.redis_config:
            self.redis_config["prefix"] = f"github:{org_name}:"

        # Initialize Redis cache
        self.cache = RedisCache(**self.redis_config)

    def _make_key(self, key_parts: List[Any]) -> str:
        """Generate a cache key from parts"""
        # Create a string representation of the key parts
        key = "-".join(str(part) for part in key_parts if part is not None)
        return key

    def get(self, key_parts: List[Any]) -> Optional[Any]:
        """Get data from cache using composite key"""
        key = self._make_key(key_parts)
        return self.cache.get(key)

    def set(self, key_parts: List[Any], data: Any, ttl: Optional[int] = None) -> bool:
        """Set data in cache using composite key"""
        key = self._make_key(key_parts)
        return self.cache.set(key, data, ttl)

    def delete(self, key_parts: List[Any]) -> bool:
        """Delete data from cache using composite key"""
        key = self._make_key(key_parts)
        return self.cache.delete(key)

    def clear_org_data(self) -> int:
        """Clear all data for this organization"""
        return self.cache.clear_all("*")

    def clear_repos_data(self) -> int:
        """Clear repository data for this organization"""
        return self.cache.clear_all("repos*")

    def get_stats(self) -> Dict[str, Any]:
        """Get cache statistics"""
        return self.cache.get_stats()

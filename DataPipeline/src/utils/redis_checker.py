"""
Redis connection checker using Airflow Redis provider
"""

import logging
from typing import Dict, Any, Optional
from airflow.providers.redis.hooks.redis import RedisHook
from airflow.hooks.base import BaseHook
from src.utils.base_connection_checker import BaseConnectionChecker

logger = logging.getLogger(__name__)


class AirflowRedisChecker(BaseConnectionChecker):
    """Redis connection checker using Airflow Redis Hook"""

    def __init__(self, conn_id: str = "redis_default"):
        """
        Initialize Redis connection checker with Airflow connection

        Args:
            conn_id: Airflow connection ID for Redis
        """
        super().__init__("redis")
        self.conn_id = conn_id
        self._hook: Optional[RedisHook] = None

    def _validate_connection_exists(self) -> None:
        """Validate that the Redis connection exists in Airflow"""
        try:
            conn = BaseHook.get_connection(self.conn_id)
            if not conn:
                raise ValueError(f"Connection '{self.conn_id}' not found in Airflow")
            if conn.conn_type != "redis":
                raise ValueError(
                    f"Connection '{self.conn_id}' is type '{conn.conn_type}', expected 'redis'"
                )
        except Exception as e:
            raise ValueError(f"Error validating Redis connection: {str(e)}")

    @property
    def hook(self) -> RedisHook:
        """Get or create Redis hook"""
        if self._hook is None:
            self._validate_connection_exists()
            self._hook = RedisHook(redis_conn_id=self.conn_id)
        return self._hook

    async def check_connection(self) -> Dict[str, Any]:
        """
        Check Redis connection and get stats

        Returns:
            Dictionary containing connection status and Redis info
        """
        try:
            # Validate connection configuration
            self._validate_connection_exists()
            conn = BaseHook.get_connection(self.conn_id)

            # Get Redis client from hook
            redis_conn = self.hook.get_conn()

            # Test connection with ping
            if not redis_conn.ping():
                return self._create_error_result("Redis ping failed")

            # Get Redis info
            info = redis_conn.info()

            return self._create_success_result(
                {
                    "connection": {
                        "id": self.conn_id,
                        "host": conn.host,
                        "port": conn.port,
                        "db": conn.extra_dejson.get("db", 0),
                        "ssl": conn.extra_dejson.get("ssl", False),
                    },
                    "stats": {
                        "version": info.get("redis_version"),
                        "used_memory": info.get("used_memory_human"),
                        "connected_clients": info.get("connected_clients"),
                        "uptime_days": info.get("uptime_in_days"),
                        "total_keys": sum(
                            info.get(f"db{i}", {}).get("keys", 0) for i in range(16)
                        ),  # Check all DBs
                    },
                    "replication": {
                        "role": info.get("role"),
                        "connected_slaves": info.get("connected_slaves", 0),
                    },
                    "persistence": {
                        "rdb_last_save": info.get("rdb_last_save_time"),
                        "aof_enabled": info.get("aof_enabled", False),
                    },
                }
            )

        except ValueError as e:
            # Configuration/validation errors
            logger.error("Redis configuration error: %s", str(e))
            return self._create_error_result(str(e))
        except Exception as e:
            # Connection/runtime errors
            logger.error("Redis connection check failed: %s", str(e))
            return self._create_error_result(f"Connection failed: {str(e)}")

    async def check_key_space(self, pattern: str = "*") -> Dict[str, Any]:
        """
        Check Redis keyspace statistics

        Args:
            pattern: Key pattern to check

        Returns:
            Dictionary containing keyspace statistics
        """
        try:
            redis_conn = self.hook.get_conn()
            keys = redis_conn.keys(pattern)

            # Get key types and TTLs
            key_stats = {
                "total": len(keys),
                "types": {},
                "ttl": {"with_ttl": 0, "no_ttl": 0, "avg_ttl": 0},
            }

            total_ttl = 0
            for key in keys:
                # Get key type
                key_type = redis_conn.type(key).decode()
                key_stats["types"][key_type] = key_stats["types"].get(key_type, 0) + 1

                # Get TTL
                ttl = redis_conn.ttl(key)
                if ttl > 0:
                    key_stats["ttl"]["with_ttl"] += 1
                    total_ttl += ttl
                else:
                    key_stats["ttl"]["no_ttl"] += 1

            # Calculate average TTL
            if key_stats["ttl"]["with_ttl"] > 0:
                key_stats["ttl"]["avg_ttl"] = total_ttl / key_stats["ttl"]["with_ttl"]

            return self._create_success_result({"keyspace": key_stats})

        except Exception as e:
            logger.error("Redis keyspace check failed: %s", str(e))
            return self._create_error_result(str(e))

    async def check_memory_usage(self) -> Dict[str, Any]:
        """
        Check Redis memory usage statistics

        Returns:
            Dictionary containing memory usage statistics
        """
        try:
            redis_conn = self.hook.get_conn()
            info = redis_conn.info("memory")

            return self._create_success_result(
                {
                    "memory": {
                        "used": info.get("used_memory_human"),
                        "peak": info.get("used_memory_peak_human"),
                        "lua": info.get("used_memory_lua_human"),
                        "fragmentation_ratio": info.get("mem_fragmentation_ratio"),
                        "allocator": info.get("mem_allocator"),
                    }
                }
            )

        except Exception as e:
            logger.error("Redis memory check failed: %s", str(e))
            return self._create_error_result(str(e))

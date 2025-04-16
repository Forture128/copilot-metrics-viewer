"""
Service-specific connection checker implementations
"""

import logging
import boto3
from typing import Dict, Any, Optional
from airflow.hooks.base import BaseHook
from src.github_collector_v2.collector import GitHubCollector
from src.utils.base_connection_checker import (
    BaseConnectionChecker,
    ConnectionCheckerManager,
)
from src.utils.redis_checker import AirflowRedisChecker

logger = logging.getLogger(__name__)


class GitHubConnectionChecker(BaseConnectionChecker):
    """GitHub API connection checker"""

    def __init__(self, collector: Optional[GitHubCollector] = None):
        """Initialize GitHub connection checker"""
        super().__init__("github")
        self.collector = collector

    async def check_connection(self) -> Dict[str, Any]:
        """Check GitHub API connection and rate limits"""
        if not self.collector:
            return self._create_skipped_result("No GitHub collector configured")

        try:
            rate_limit = await self.collector.check_rate_limit()
            return self._create_success_result(
                {
                    "rate_limit": {
                        "remaining": rate_limit.get("remaining"),
                        "limit": rate_limit.get("limit"),
                        "reset_at": rate_limit.get("resetAt"),
                    }
                }
            )
        except Exception as e:
            return self._create_error_result(str(e))


class S3ConnectionChecker(BaseConnectionChecker):
    """S3 connection checker using Airflow AWS connection"""

    def __init__(self, conn_id: str = "aws_default"):
        """
        Initialize S3 connection checker

        Args:
            conn_id: Airflow connection ID for AWS
        """
        super().__init__("s3")
        self.conn_id = conn_id
        self._client = None

    def _validate_connection_exists(self) -> None:
        """Validate that the AWS connection exists in Airflow"""
        try:
            conn = BaseHook.get_connection(self.conn_id)
            if not conn:
                raise ValueError(f"Connection '{self.conn_id}' not found in Airflow")
            if conn.conn_type != "aws":
                raise ValueError(
                    f"Connection '{self.conn_id}' is type '{conn.conn_type}', expected 'aws'"
                )
            if not conn.login or not conn.password:
                raise ValueError(
                    f"Connection '{self.conn_id}' is missing AWS credentials"
                )
        except Exception as e:
            raise ValueError(f"Error validating AWS connection: {str(e)}")

    def _get_client(self) -> Any:
        """Get or create S3 client"""
        if self._client is None:
            self._validate_connection_exists()
            conn = BaseHook.get_connection(self.conn_id)

            # Get configuration from connection
            config = {
                "aws_access_key_id": conn.login,
                "aws_secret_access_key": conn.password,
                "region_name": conn.extra_dejson.get("region_name", "us-east-1"),
            }
            # Add optional configurations
            if "endpoint_url" in conn.extra_dejson:
                config["endpoint_url"] = conn.extra_dejson["endpoint_url"]
            else:
                config["endpoint_url"] = f"http://{conn.host}:{conn.port}"
            if "verify" in conn.extra_dejson:
                config["verify"] = conn.extra_dejson["verify"]
            else:
                config["verify"] = False
            self._client = boto3.client("s3", **config)
        return self._client

    async def check_connection(self) -> Dict[str, Any]:
        """Check S3 connection"""
        try:
            # Validate connection configuration
            self._validate_connection_exists()
            conn = BaseHook.get_connection(self.conn_id)

            # Get S3 client
            s3_client = self._get_client()

            # Test connection by listing buckets
            response = s3_client.list_buckets()
            buckets = [bucket["Name"] for bucket in response["Buckets"]]

            return self._create_success_result(
                {
                    "connection": {
                        "id": self.conn_id,
                        "region": conn.login,
                        "endpoint": conn.extra_dejson.get("endpoint_url", "default"),
                    },
                    "buckets": {
                        "count": len(buckets),
                        "names": buckets[:10],  # List first 10 buckets
                    },
                }
            )

        except ValueError as e:
            # Configuration/validation errors
            logger.error("AWS configuration error: %s", str(e))
            return self._create_error_result(str(e))
        except Exception as e:
            # Connection/runtime errors
            logger.error("S3 connection check failed: %s", str(e))
            return self._create_error_result(f"Connection failed: {str(e)}")


def create_connection_manager(
    github_collector: Optional[GitHubCollector] = None,
    redis_conn_id: str = "redis_default",
    aws_conn_id: str = "aws_default",
) -> ConnectionCheckerManager:
    """
    Create a connection checker manager with all available checkers

    Args:
        github_collector: GitHubCollector instance
        redis_conn_id: Airflow connection ID for Redis
        aws_conn_id: Airflow connection ID for AWS

    Returns:
        Configured ConnectionCheckerManager
    """
    manager = ConnectionCheckerManager()

    # Add available checkers
    manager.add_checker(GitHubConnectionChecker(github_collector))
    manager.add_checker(AirflowRedisChecker(redis_conn_id))
    manager.add_checker(S3ConnectionChecker(aws_conn_id))

    return manager

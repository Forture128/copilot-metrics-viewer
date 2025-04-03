"""
Utility functions for GitHub data collection.
"""

import os
import json
import logging
import asyncio
import time
from datetime import datetime, timedelta
from pathlib import Path
from typing import Dict, Any
from enum import Enum
import signal
import sys

from airflow.models import Variable
from airflow.exceptions import AirflowException

from src.github_collector.collector import GitHubCollector, GitHubObjectEncoder
from src.github_collector.extended_collector import ExtendedGitHubCollector
from src.github_collector.graphql_collector import GitHubGraphQLCollector

# Configure logging with more detailed format
logger = logging.getLogger(__name__)
logger.setLevel(logging.INFO)

# Add handler if not already present
if not logger.handlers:
    handler = logging.StreamHandler()
    formatter = logging.Formatter(
        "[%(asctime)s] {%(filename)s:%(lineno)d} %(levelname)s - %(message)s",
        datefmt="%Y-%m-%d, %H:%M:%S UTC",
    )
    handler.setFormatter(formatter)
    logger.addHandler(handler)


class AsyncOperationMonitor:
    """Monitor async operations with timeout and progress tracking"""

    def __init__(self, timeout_seconds: int = 3600, check_interval: int = 5):
        self.start_time = time.time()
        self.timeout_seconds = timeout_seconds
        self.check_interval = check_interval
        self.last_progress_time = self.start_time
        self.progress_count = 0
        self.is_running = True

    def update_progress(self, count: int = 1) -> None:
        """Update progress counter"""
        self.progress_count += count
        self.last_progress_time = time.time()
        logger.info(
            "Async operation progress: %d items processed in %.2f seconds",
            self.progress_count,
            self.last_progress_time - self.start_time,
        )

    def check_timeout(self) -> bool:
        """Check if operation has timed out"""
        elapsed = time.time() - self.start_time
        if elapsed > self.timeout_seconds:
            logger.error(
                "Async operation timed out after %.2f seconds (limit: %d)",
                elapsed,
                self.timeout_seconds,
            )
            return True
        return False

    def check_stagnation(self) -> bool:
        """Check if operation has stagnated"""
        if time.time() - self.last_progress_time > self.check_interval * 2:
            logger.warning(
                "Async operation stagnated for %.2f seconds",
                time.time() - self.last_progress_time,
            )
            return True
        return False

    def stop(self) -> None:
        """Stop the monitor"""
        self.is_running = False


class CollectionMode(Enum):
    BASIC = "basic"
    EXTENDED = "extended"
    ALL = "all"


class GitHubCollectorConfig:
    """Configuration class for GitHub collector"""

    def __init__(self, **context):
        logger.info("Initializing GitHub collector configuration")
        self.org_name = Variable.get("github_org", default_var="default-org")
        self.output_dir = Variable.get("github_data_dir", default_var="data/github")
        self.concurrent_requests = int(
            Variable.get("github_concurrent_requests", default_var="5")
        )
        self.use_cache = (
            Variable.get("github_use_cache", default_var="true").lower() == "true"
        )
        self.retention_days = int(
            Variable.get("github_retention_days", default_var="30")
        )
        self.async_timeout = int(
            Variable.get("github_async_timeout", default_var="3600")
        )
        # Get collection mode from context or Airflow variable
        collection_mode = context.get("collection_mode") or Variable.get(
            "github_collection_mode"
        )
        self.collection_mode = CollectionMode(collection_mode)

        logger.info(
            "Configuration initialized: org=%s, mode=%s, concurrent=%d, cache=%s, timeout=%d",
            self.org_name,
            self.collection_mode.value,
            self.concurrent_requests,
            self.use_cache,
            self.async_timeout,
        )
        self.validate()

    def validate(self) -> None:
        """Validate configuration values"""
        logger.debug("Validating configuration")
        if not self.org_name:
            raise ValueError("GitHub organization name is required")
        if not self.output_dir:
            raise ValueError("Output directory is required")
        if self.concurrent_requests < 1:
            raise ValueError("Concurrent requests must be at least 1")
        if self.retention_days < 1:
            raise ValueError("Retention days must be at least 1")
        if self.async_timeout < 1:
            raise ValueError("Async timeout must be at least 1 second")
        if self.collection_mode not in CollectionMode:
            raise ValueError(f"Invalid collection mode: {self.collection_mode}")
        logger.debug("Configuration validation successful")


class GitHubCollectorService:
    """Service class to handle GitHub data collection"""

    def __init__(self, config: GitHubCollectorConfig):
        logger.info("Initializing GitHub collector service")
        self.config = config
        self.base_collector = GitHubCollector(
            concurrent_requests=config.concurrent_requests, use_cache=config.use_cache
        )
        self.extended_collector = ExtendedGitHubCollector(
            concurrent_requests=config.concurrent_requests, use_cache=config.use_cache
        )
        self.graphql_collector = GitHubGraphQLCollector(
            token=os.getenv("GITHUB_TOKEN"), use_cache=config.use_cache
        )
        self.monitor = AsyncOperationMonitor(timeout_seconds=config.async_timeout)
        logger.info("GitHub collector service initialized")

    def collect_data(self) -> Dict[str, Any]:
        """Collect data based on configured mode"""
        logger.info(
            "Starting data collection for organization %s in %s mode",
            self.config.org_name,
            self.config.collection_mode.value,
        )

        try:
            if self.config.collection_mode == CollectionMode.ALL:
                data = self.collect_all_data()
            elif self.config.collection_mode == CollectionMode.BASIC:
                data = self.collect_basic_data()
            else:  # EXTENDED mode
                data = self.collect_extended_data()

            logger.info(
                "Data collection completed for organization %s in %s mode",
                self.config.org_name,
                self.config.collection_mode.value,
            )
            return data

        except Exception as e:
            logger.error(
                "Failed to collect data for organization %s in %s mode: %s",
                self.config.org_name,
                self.config.collection_mode.value,
                str(e),
            )
            raise
        finally:
            self.monitor.stop()

    def collect_basic_data(self) -> Dict[str, Any]:
        """Collect basic GitHub data"""
        logger.info("[BASIC] Starting data collection")
        data = self.base_collector.collect_all_data_sync(self.config.org_name)
        logger.info("[BASIC] Data collection completed")
        return data

    def collect_extended_data(self) -> Dict[str, Any]:
        """Collect extended GitHub data"""
        logger.info("[EXTENDED] Starting data collection")
        try:
            # Set up signal handlers for graceful shutdown
            def signal_handler(signum, frame):
                logger.warning("Received shutdown signal, stopping collection...")
                self.monitor.stop()
                sys.exit(0)

            signal.signal(signal.SIGINT, signal_handler)
            signal.signal(signal.SIGTERM, signal_handler)

            # Start async collection with monitoring
            loop = asyncio.new_event_loop()
            asyncio.set_event_loop(loop)

            try:
                logger.info("[EXTENDED] Starting async collection")
                data = loop.run_until_complete(
                    self._collect_extended_data_with_monitoring()
                )
                logger.info("[EXTENDED] Async collection completed successfully")
                return data
            finally:
                loop.close()

        except asyncio.TimeoutError:
            logger.error("[EXTENDED] Collection timed out")
            raise
        except Exception as e:
            logger.error("[EXTENDED] Collection failed: %s", str(e))
            raise

    async def _collect_extended_data_with_monitoring(self) -> Dict[str, Any]:
        """Collect extended data with monitoring"""
        try:
            # Start the collection task
            collection_task = asyncio.create_task(
                self.extended_collector.collect_extended_data_async(
                    self.config.org_name
                )
            )

            # Monitor the task
            while not collection_task.done():
                if self.monitor.check_timeout():
                    collection_task.cancel()
                    raise asyncio.TimeoutError("Collection timed out")

                if self.monitor.check_stagnation():
                    logger.warning("[EXTENDED] Collection appears to be stagnated")

                await asyncio.sleep(self.monitor.check_interval)

            # Get the result
            result = await collection_task
            self.monitor.update_progress(1)  # Mark completion
            return result

        except asyncio.CancelledError:
            logger.error("[EXTENDED] Collection was cancelled")
            raise
        except Exception as e:
            logger.error("[EXTENDED] Collection failed: %s", str(e))
            raise

    def collect_all_data(self) -> Dict[str, Any]:
        """Collect all available GitHub data"""
        logger.info("[ALL] Starting data collection")

        # Collect data from all collectors
        logger.info("[ALL] Collecting basic data")
        base_data = self.collect_basic_data()

        logger.info("[ALL] Collecting extended data")
        extended_data = self.collect_extended_data()

        logger.info("[ALL] Collecting GraphQL data")
        graphql_data = self.graphql_collector.collect_organization_data_sync(
            self.config.org_name
        )

        # Combine results
        logger.info("[ALL] Combining collected data")
        result = {
            "base": base_data,
            "extended": extended_data,
            "graphql": graphql_data,
            "metadata": {
                "timestamp": datetime.now().isoformat(),
                "target": self.config.org_name,
                "collector_version": "1.0.0",
                "collection_mode": self.config.collection_mode.value,
                "collection_duration": time.time() - self.monitor.start_time,
            },
        }
        logger.info("[ALL] Comprehensive data collection completed")
        return result

    def save_data(self, data: Dict[str, Any]) -> str:
        """Save collected data to file"""
        logger.info("Saving collected data to file")
        timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
        output_path = Path(self.config.output_dir)
        output_path.mkdir(parents=True, exist_ok=True)

        output_file = output_path / f"{self.config.org_name}_{timestamp}.json"
        with open(output_file, "w") as f:
            json.dump(data, f, indent=2, cls=GitHubObjectEncoder)

        logger.info("Data saved to %s", output_file)
        return str(output_file)


def cleanup_old_files(output_dir: str, retention_days: int) -> None:
    """Remove files older than retention_days"""
    logger.info("Starting cleanup of old files")
    try:
        output_path = Path(output_dir)
        if not output_path.exists():
            logger.info("Output directory does not exist, skipping cleanup")
            return

        cutoff_date = datetime.now() - timedelta(days=retention_days)
        files_removed = 0
        for file_path in output_path.glob("*.json"):
            if file_path.stat().st_mtime < cutoff_date.timestamp():
                file_path.unlink()
                files_removed += 1
                logger.info("Removed old file: %s", file_path)

        logger.info("Cleanup completed: removed %d files", files_removed)
    except Exception as e:
        logger.error("Failed to cleanup old files: %s", str(e))
        raise


def validate_collected_data(data: Dict[str, Any]) -> bool:
    """Validate collected GitHub data"""
    logger.debug("Validating collected data")
    required_fields = ["organization", "repositories"]
    is_valid = all(field in data for field in required_fields)
    if not is_valid:
        logger.error("Data validation failed: missing required fields")
    return is_valid


def collect_github_data(**context) -> str:
    """Collect GitHub data with proper error handling and validation

    Args:
        **context: Airflow context including:
            - collection_mode: Optional collection mode ("basic", "extended", "all")
    """
    logger.info("Starting GitHub data collection task")
    try:
        # Get configuration with context
        config = GitHubCollectorConfig(**context)

        # Initialize service and collect data
        service = GitHubCollectorService(config)
        data = service.collect_data()

        # Validate collected data
        if not validate_collected_data(data):
            raise ValueError("Collected data is missing required fields")

        # Save data
        output_file = service.save_data(data)

        # Cleanup old files
        cleanup_old_files(config.output_dir, config.retention_days)

        logger.info("GitHub data collection task completed successfully")
        return f"Data collected and saved to {output_file}"

    except Exception as e:
        logger.error("Failed to collect GitHub data: %s", str(e))
        raise AirflowException("GitHub data collection failed") from e

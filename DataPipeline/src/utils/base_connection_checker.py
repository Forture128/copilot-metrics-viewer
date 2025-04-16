"""
Base class for connection checkers with async support
"""

import logging
import asyncio
from abc import ABC, abstractmethod
from typing import Dict, Any, Optional, List

logger = logging.getLogger(__name__)


class BaseConnectionChecker(ABC):
    """Base class for connection checkers"""

    def __init__(self, service_name: str):
        """
        Initialize base connection checker

        Args:
            service_name: Name of the service being checked
        """
        self.service_name = service_name

    @abstractmethod
    async def check_connection(self) -> Dict[str, Any]:
        """
        Check connection to the service

        Returns:
            Dictionary containing connection status and details
        """
        pass

    def _create_success_result(self, details: Dict[str, Any]) -> Dict[str, Any]:
        """Create a standardized success result"""
        return {
            "status": "success",
            "service": self.service_name,
            "details": details,
            "error": None,
        }

    def _create_error_result(self, error: str) -> Dict[str, Any]:
        """Create a standardized error result"""
        return {
            "status": "error",
            "service": self.service_name,
            "details": None,
            "error": str(error),
        }

    def _create_skipped_result(self, reason: str) -> Dict[str, Any]:
        """Create a standardized skipped result"""
        return {
            "status": "skipped",
            "service": self.service_name,
            "details": None,
            "error": None,
            "reason": reason,
        }


class ConnectionCheckerManager:
    """Manager class to handle multiple connection checkers"""

    def __init__(self, checkers: Optional[List[BaseConnectionChecker]] = None):
        """
        Initialize connection checker manager

        Args:
            checkers: List of connection checkers to manage
        """
        self.checkers = checkers or []

    def add_checker(self, checker: BaseConnectionChecker) -> None:
        """Add a connection checker to the manager"""
        self.checkers.append(checker)

    async def check_single(self, checker: BaseConnectionChecker) -> Dict[str, Any]:
        """
        Check a single connection with error handling

        Args:
            checker: Connection checker to use

        Returns:
            Connection check results
        """
        try:
            result = await checker.check_connection()
            logger.info(
                "%s connection check: %s",
                checker.service_name.upper(),
                result["status"],
            )
            return result
        except Exception as e:
            error_msg = f"Error checking {checker.service_name} connection: {str(e)}"
            logger.error(error_msg)
            return checker._create_error_result(error_msg)

    async def check_all(self, timeout: float = 30.0) -> Dict[str, List[Dict[str, Any]]]:
        """
        Check all connections in parallel with timeout

        Args:
            timeout: Maximum time to wait for all checks (in seconds)

        Returns:
            Dictionary containing results grouped by status
        """
        try:
            # Create tasks for all checkers
            tasks = [self.check_single(checker) for checker in self.checkers]

            # Run all checks in parallel with timeout
            results = await asyncio.wait_for(
                asyncio.gather(*tasks, return_exceptions=True), timeout=timeout
            )

            # Process results
            processed_results = {"success": [], "error": [], "skipped": []}

            for result in results:
                if isinstance(result, Exception):
                    # Handle unexpected exceptions
                    processed_results["error"].append(
                        {"status": "error", "service": "unknown", "error": str(result)}
                    )
                else:
                    # Add result to appropriate category
                    processed_results[result["status"]].append(result)

            # Log summary
            self._log_check_summary(processed_results)
            return processed_results

        except asyncio.TimeoutError:
            logger.error("Connection checks timed out after %s seconds", timeout)
            return {
                "success": [],
                "error": [
                    {
                        "status": "error",
                        "service": "all",
                        "error": f"Timeout after {timeout} seconds",
                    }
                ],
                "skipped": [],
            }
        except Exception as e:
            logger.error("Unexpected error during connection checks: %s", str(e))
            return {
                "success": [],
                "error": [{"status": "error", "service": "all", "error": str(e)}],
                "skipped": [],
            }

    def _log_check_summary(self, results: Dict[str, List[Dict[str, Any]]]) -> None:
        """Log a summary of connection check results"""
        logger.info("Connection Check Summary:")
        logger.info("------------------------")
        logger.info("Successful checks: %d", len(results["success"]))
        logger.info("Failed checks: %d", len(results["error"]))
        logger.info("Skipped checks: %d", len(results["skipped"]))

        if results["error"]:
            logger.info("\nFailed Services:")
            for result in results["error"]:
                logger.error(
                    "- %s: %s", result["service"], result.get("error", "Unknown error")
                )

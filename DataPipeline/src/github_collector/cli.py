"""
GitHub Collector CLI - Command-line interface for GitHub data collection

This CLI provides a unified interface for collecting GitHub data using the
various collectors implemented in this package. It supports both REST API and
GraphQL API data collection with various options.
"""

import argparse
import os
import sys
import logging
import json
from pathlib import Path
from datetime import datetime
from typing import Dict, Any

# Local imports
from src.github_collector.collector import (
    GitHubCollector,
    DEFAULT_CONCURRENT_REQUESTS,
    GitHubObjectEncoder,
)
from src.github_collector.extended_collector import ExtendedGitHubCollector
from src.github_collector.graphql_collector import GitHubGraphQLCollector
from src.utils.loggers import get_logger, configure_logger

# Set up logger
logger = get_logger("github_collector_cli")


def add_common_arguments(parser: argparse.ArgumentParser) -> None:
    """Add common arguments to a parser"""
    parser.add_argument(
        "--token",
        help="GitHub personal access token (will use GITHUB_TOKEN env var if not provided)",
    )
    parser.add_argument("--org", help="GitHub organization name")
    parser.add_argument("--enterprise", help="GitHub enterprise name")
    parser.add_argument(
        "--repo",
        help="GitHub repository in owner/repo format (e.g., 'octocat/Hello-World')",
    )
    parser.add_argument(
        "--output-dir", default="data/github", help="Directory to save output files"
    )
    parser.add_argument(
        "--no-cache", action="store_true", help="Disable caching of API responses"
    )
    parser.add_argument(
        "--log-level",
        choices=["DEBUG", "INFO", "WARNING", "ERROR", "CRITICAL"],
        default="INFO",
        help="Set the logging level",
    )


def add_concurrent_argument(parser: argparse.ArgumentParser) -> None:
    """Add concurrent requests argument to a parser"""
    parser.add_argument(
        "--concurrent",
        type=int,
        default=DEFAULT_CONCURRENT_REQUESTS,
        help="Number of concurrent API requests",
    )


def setup_parser() -> argparse.ArgumentParser:
    """Set up command line argument parser"""
    parser = argparse.ArgumentParser(
        description="GitHub Data Collector - CLI for collecting GitHub data"
    )

    # Subparsers for different collection modes
    subparsers = parser.add_subparsers(dest="command", help="Collection command")

    # Base collector (REST API)
    base_parser = subparsers.add_parser(
        "base", help="Collect basic GitHub data using REST API"
    )
    add_common_arguments(base_parser)
    add_concurrent_argument(base_parser)

    # Extended collector (REST API with more data)
    extended_parser = subparsers.add_parser(
        "extended", help="Collect extended GitHub data using REST API"
    )
    add_common_arguments(extended_parser)
    add_concurrent_argument(extended_parser)

    # GraphQL collector
    graphql_parser = subparsers.add_parser(
        "graphql", help="Collect GitHub data using GraphQL API"
    )
    add_common_arguments(graphql_parser)

    # Combined collector (all methods)
    combined_parser = subparsers.add_parser(
        "all", help="Collect GitHub data using all available methods"
    )
    add_common_arguments(combined_parser)
    add_concurrent_argument(combined_parser)

    return parser


def validate_args(args: argparse.Namespace) -> bool:
    """Validate command line arguments"""
    # Check for required token
    if not args.token and not os.getenv("GITHUB_TOKEN"):
        logger.error(
            "GitHub token is required. Either use --token or set GITHUB_TOKEN env var."
        )
        return False

    # Check for required target (org, enterprise, or repo)
    if not args.org and not args.enterprise and not args.repo:
        logger.error("You must specify either --org, --enterprise, or --repo.")
        return False

    # Make sure the output directory can be created
    try:
        Path(args.output_dir).mkdir(parents=True, exist_ok=True)
    except Exception as e:
        logger.error(f"Failed to create output directory: {str(e)}")
        return False

    return True


def generate_output_filename(args: argparse.Namespace, collector_type: str) -> str:
    """Generate output filename based on command line arguments"""
    timestamp = datetime.now().strftime("%Y%m%d%H%M%S")
    target = args.org or args.enterprise or args.repo.replace("/", "_")
    return Path(args.output_dir) / f"github_{collector_type}_{target}_{timestamp}.json"


def collect_data_with_collector(
    collector_type: str,
    collector_class,
    args: argparse.Namespace,
    method_name: str = "collect_all_data_sync",
) -> Dict[str, Any]:
    """Generic function to collect data using a specified collector"""
    collector_kwargs = {
        "token": args.token,
        "use_cache": not args.no_cache,
    }

    # Add concurrent requests if the collector supports it
    if hasattr(collector_class, "concurrent_requests"):
        collector_kwargs["concurrent_requests"] = args.concurrent

    collector = collector_class(**collector_kwargs)
    collection_method = getattr(collector, method_name)

    if args.org:
        logger.info(f"Collecting {collector_type} data for organization: {args.org}")
        data = collection_method(args.org)
    elif args.repo:
        owner, repo = args.repo.split("/")
        logger.info(f"Collecting {collector_type} data for repository: {args.repo}")
        data = {
            "error": f"Repository-specific collection not implemented for {collector_type} collector"
        }
    else:
        data = {"error": "Neither organization nor repository specified"}

    output_file = generate_output_filename(args, collector_type)
    collector.save_data_to_json(data, output_file)

    logger.info(f"{collector_type.capitalize()} data saved to {output_file}")
    return data


def collect_base_data(args: argparse.Namespace) -> Dict[str, Any]:
    """Collect base GitHub data using REST API"""
    return collect_data_with_collector("base", GitHubCollector, args)


def collect_extended_data(args: argparse.Namespace) -> Dict[str, Any]:
    """Collect extended GitHub data using REST API"""
    return collect_data_with_collector(
        "extended", ExtendedGitHubCollector, args, "collect_extended_data_sync"
    )


def collect_graphql_data(args: argparse.Namespace) -> Dict[str, Any]:
    """Collect GitHub data using GraphQL API"""
    return collect_data_with_collector(
        "graphql", GitHubGraphQLCollector, args, "collect_organization_data_sync"
    )


def collect_all_data(args: argparse.Namespace) -> Dict[str, Any]:
    """Collect GitHub data using all available methods"""
    logger.info("Collecting data using all available methods")

    # Collect data using all three methods
    base_data = collect_base_data(args)
    extended_data = collect_extended_data(args)
    graphql_data = collect_graphql_data(args)

    # Combine results
    combined_data = {
        "base": base_data,
        "extended": extended_data,
        "graphql": graphql_data,
        "metadata": {
            "timestamp": datetime.now().isoformat(),
            "target": args.org or args.enterprise or args.repo,
            "collector_version": "1.0.0",
        },
    }

    # Save combined results
    output_file = generate_output_filename(args, "combined")
    try:
        with open(output_file, "w") as f:
            json.dump(combined_data, f, indent=2, cls=GitHubObjectEncoder)
        logger.info(f"Combined data saved to {output_file}")
    except Exception as e:
        logger.error(f"Failed to save combined data: {str(e)}")

    return combined_data


def main():
    """Main CLI entrypoint"""
    parser = setup_parser()
    args = parser.parse_args()

    # Configure logging
    configure_logger(level=getattr(logging, args.log_level))

    # Validate arguments
    if not validate_args(args):
        sys.exit(1)

    # Handle command
    if args.command == "base":
        collect_base_data(args)
    elif args.command == "extended":
        collect_extended_data(args)
    elif args.command == "graphql":
        collect_graphql_data(args)
    elif args.command == "all":
        collect_all_data(args)
    else:
        parser.print_help()
        sys.exit(1)

    sys.exit(0)


if __name__ == "__main__":
    main()

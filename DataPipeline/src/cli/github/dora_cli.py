#!/usr/bin/env python3
"""
GitHub DORA Metrics CLI
"""

import click
import glob
import os
from pathlib import Path
import logging
from dotenv import load_dotenv
from src.github_collector_v2.dora_metrics import (
    load_github_data,
    calculate_dora_metrics,
    get_four_key_metrics,
    classify_performance_level,
)
from src.utils.loggers import get_logger

# Set up logging
logger = get_logger("github_collector_v2.dora_cli")


@click.group()
def cli():
    """DORA Metrics calculation CLI"""
    load_dotenv()


@cli.command()
@click.argument("input_file", type=click.Path(exists=True))
@click.option("--output", "-o", type=click.Path(), help="Path to save the metrics to")
@click.option(
    "--days",
    "-d",
    type=int,
    default=90,
    help="Time period in days to calculate metrics",
)
def calculate(input_file: str, output: str, days: int):
    """Calculate DORA metrics from GitHub data file"""
    try:
        # Load the data
        logger.info(f"Loading data from {input_file}")
        data = load_github_data(input_file)

        # Determine output file name if not specified
        if not output:
            input_path = Path(input_file)
            output = str(input_path.parent / f"dora_metrics_{input_path.stem}.json")

        # Calculate metrics
        logger.info(f"Calculating DORA metrics for time period of {days} days")
        metrics = calculate_dora_metrics(data, days, output)

        # Extract and print the four key metrics
        four_keys = get_four_key_metrics(metrics)
        performance = classify_performance_level(four_keys)

        # Print summary
        org_name = metrics.get("organization", "unknown")
        total_repos = metrics.get("organization_summary", {}).get(
            "total_repositories", 0
        )

        click.echo(
            click.style(f"\nDORA Metrics Summary for {org_name}", fg="green", bold=True)
        )
        click.echo(f"Time period: {days} days")
        click.echo(f"Total repositories: {total_repos}")

        click.echo(click.style("\nFour Key Metrics:", fg="blue", bold=True))

        # Format each metric with its performance level and color
        df = four_keys["deployment_frequency_per_week"]
        df_level = performance["deployment_frequency"]
        df_color = (
            "green"
            if df_level in ["Elite", "High"]
            else ("yellow" if df_level == "Medium" else "red")
        )
        click.echo(
            f"1. Deployment Frequency: {df:.2f} per week "
            + click.style(f"({df_level})", fg=df_color)
        )

        lt = four_keys["lead_time_for_changes_hours"]
        lt_level = performance["lead_time_for_changes"]
        lt_color = (
            "green"
            if lt_level in ["Elite", "High"]
            else ("yellow" if lt_level == "Medium" else "red")
        )
        click.echo(
            f"2. Lead Time for Changes: {lt:.2f} hours "
            + click.style(f"({lt_level})", fg=lt_color)
        )

        cfr = four_keys["change_failure_rate"] * 100
        cfr_level = performance["change_failure_rate"]
        cfr_color = (
            "green"
            if cfr_level in ["Elite", "High"]
            else ("yellow" if cfr_level == "Medium" else "red")
        )
        click.echo(
            f"3. Change Failure Rate: {cfr:.2f}% "
            + click.style(f"({cfr_level})", fg=cfr_color)
        )

        mttr = four_keys["mean_time_to_recover_hours"]
        mttr_level = performance["mean_time_to_recover"]
        mttr_color = (
            "green"
            if mttr_level in ["Elite", "High"]
            else ("yellow" if mttr_level == "Medium" else "red")
        )
        click.echo(
            f"4. Mean Time to Recover: {mttr:.2f} hours "
            + click.style(f"({mttr_level})", fg=mttr_color)
        )

        # Print repository details
        click.echo(click.style("\nRepository Details:", fg="blue", bold=True))
        for repo, data in metrics.get("repositories", {}).items():
            if "error" in data:
                click.echo(
                    f"- {repo}: " + click.style("Error collecting data", fg="red")
                )
                continue

            pr_count = data.get("pr_count", 0)
            deploy_count = data.get("deployment_count", 0)
            click.echo(f"- {repo}: {pr_count} PRs, {deploy_count} deployments")

        if output:
            click.echo(
                click.style(f"\nDetailed metrics saved to: {output}", fg="green")
            )

    except Exception as e:
        logger.error(f"Error calculating DORA metrics: {str(e)}")
        click.echo(click.style(f"Error: {str(e)}", fg="red"))


@cli.command()
@click.argument("directory", type=click.Path(exists=True, file_okay=False))
@click.option(
    "--pattern", "-p", default="*github_data_*.json", help="File pattern to match"
)
@click.option("--output-dir", type=click.Path(), help="Directory to save metrics to")
@click.option(
    "--days",
    "-d",
    type=int,
    default=90,
    help="Time period in days to calculate metrics",
)
def batch_calculate(directory: str, pattern: str, output_dir: str, days: int):
    """Calculate DORA metrics for multiple files in a directory"""
    try:
        # Find matching files
        search_pattern = os.path.join(directory, pattern)
        files = glob.glob(search_pattern)

        if not files:
            click.echo(
                click.style(
                    f"No files found matching pattern {pattern} in {directory}",
                    fg="yellow",
                )
            )
            return

        click.echo(f"Found {len(files)} files to process")

        # Determine output directory
        if not output_dir:
            output_dir = os.path.join(directory, "dora_metrics")

        # Create output directory if it doesn't exist
        os.makedirs(output_dir, exist_ok=True)

        # Process each file
        for i, file_path in enumerate(files):
            file_name = os.path.basename(file_path)
            click.echo(f"Processing file {i + 1}/{len(files)}: {file_name}")

            # Determine output file name
            base_name = os.path.splitext(file_name)[0]
            output_file = os.path.join(output_dir, f"dora_{base_name}.json")

            try:
                # Load and calculate metrics
                data = load_github_data(file_path)
                calculate_dora_metrics(data, days, output_file)
                click.echo(click.style(f"  Metrics saved to {output_file}", fg="green"))
            except Exception as e:
                click.echo(
                    click.style(f"  Error processing {file_name}: {str(e)}", fg="red")
                )

        click.echo(
            click.style(
                f"Batch processing complete. Results saved to {output_dir}", fg="green"
            )
        )

    except Exception as e:
        logger.error(f"Error in batch processing: {str(e)}")
        click.echo(click.style(f"Error: {str(e)}", fg="red"))


def main():
    """CLI entry point"""
    cli()


if __name__ == "__main__":
    main()

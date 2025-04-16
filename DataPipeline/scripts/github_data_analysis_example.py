#!/usr/bin/env python3
"""
GitHub Data Analysis Example

This script demonstrates how to use the github_data_utils module to analyze
team and member data from GitHub.
"""

import os
import sys
import pandas as pd
import matplotlib.pyplot as plt
from pathlib import Path
from dotenv import load_dotenv

# Add the project root to the Python path
script_dir = os.path.dirname(os.path.abspath(__file__))
project_root = os.path.dirname(script_dir)
sys.path.insert(0, project_root)

# Import utilities
from src.utils.github_data_utils import (
    analyze_github_data,
    get_top_contributors,
    get_teams_by_activity,
    find_latest_data_files,
)
from src.utils.loggers import get_logger
from src.utils.helper_functions import ensure_output_dir

# Set up logging
logger = get_logger("github_data_analysis_example")


def main():
    """Main function to demonstrate GitHub data analysis."""
    load_dotenv()  # Load environment variables from .env file

    # Set up directories
    data_dir = os.path.join(project_root, "data", "github")
    output_dir = os.path.join(project_root, "data", "github", "analysis")

    # Ensure output directory exists
    output_dir = ensure_output_dir(output_dir)

    # Get organization name
    org_name = os.getenv("GITHUB_ORG", "moneyforward")

    # Find latest data files
    logger.info(f"Looking for latest data files for organization: {org_name}")
    team_file, member_file = find_latest_data_files(data_dir, org_name)

    if not team_file and not member_file:
        logger.error(f"No data files found for {org_name} in {data_dir}")
        print(f"No data files found for {org_name}. Please run the collector first.")
        return

    # Log found files
    if team_file:
        logger.info(f"Found team file: {os.path.basename(team_file)}")
    if member_file:
        logger.info(f"Found member file: {os.path.basename(member_file)}")

    # Analyze GitHub data
    results = analyze_github_data(
        team_file_path=team_file,
        member_file_path=member_file,
        org_name=org_name,
        output_dir=output_dir,
    )

    # Print summary
    summary = results["summary"]
    print("\n=== GitHub Data Analysis Summary ===")
    print(f"Organization: {summary.get('organization')}")
    print(f"Teams: {summary.get('team_count')}")
    print(f"Members: {summary.get('member_count')}")
    print(f"Unique members: {summary.get('unique_member_count')}")

    # Show contribution metrics if available
    if "total_commits" in summary:
        print("\n=== Contribution Metrics ===")
        print(f"Total commits: {summary['total_commits']:,}")
        print(f"Total PRs: {summary['total_prs']:,}")
        print(f"Total issues: {summary['total_issues']:,}")
        print(f"Total reviews: {summary['total_reviews']:,}")

    # Show output files
    print("\n=== Output Files ===")
    for key in summary:
        if key.endswith("_path"):
            print(f"{key}: {summary[key]}")

    # Visualization example if matplotlib is available
    try:
        # Top contributors visualization
        if "member_df" in results and not results["member_df"].empty:
            member_df = results["member_df"]
            top_contributors = get_top_contributors(member_df, n=10)

            if (
                not top_contributors.empty
                and "total_contributions" in top_contributors.columns
            ):
                print("\n=== Top 10 Contributors ===")
                for _, row in top_contributors.iterrows():
                    name = row.get("member_name") or row["member_login"]
                    contributions = int(row["total_contributions"])
                    print(f"{name}: {contributions:,} contributions")

                # Create visualization
                plt.figure(figsize=(12, 6))
                plt.barh(
                    [
                        row.get("member_name") or row["member_login"]
                        for _, row in top_contributors.iterrows()
                    ],
                    top_contributors["total_contributions"],
                    color="skyblue",
                )
                plt.xlabel("Total Contributions")
                plt.ylabel("Member")
                plt.title(f"Top 10 Contributors - {org_name}")
                plt.tight_layout()

                # Save visualization
                chart_path = os.path.join(
                    output_dir, f"{org_name}_top_contributors.png"
                )
                plt.savefig(chart_path)
                print(f"\nTop contributors chart saved to: {chart_path}")

        # Team activity visualization
        if "combined_df" in results and not results["combined_df"].empty:
            combined_df = results["combined_df"]
            team_activity = get_teams_by_activity(combined_df)

            if (
                not team_activity.empty
                and "total_contributions" in team_activity.columns
            ):
                top_teams = team_activity.head(10)

                print("\n=== Top 10 Teams by Activity ===")
                for team_name, row in top_teams.iterrows():
                    contributions = int(row["total_contributions"])
                    members = int(row["member_count"])
                    per_member = round(row["contributions_per_member"], 1)
                    print(
                        f"{team_name}: {contributions:,} contributions ({members} members, {per_member} per member)"
                    )

                # Create visualization
                plt.figure(figsize=(12, 6))
                plt.barh(
                    top_teams.index,
                    top_teams["total_contributions"],
                    color="lightgreen",
                )
                plt.xlabel("Total Contributions")
                plt.ylabel("Team")
                plt.title(f"Top 10 Teams by Activity - {org_name}")
                plt.tight_layout()

                # Save visualization
                chart_path = os.path.join(output_dir, f"{org_name}_top_teams.png")
                plt.savefig(chart_path)
                print(f"\nTop teams chart saved to: {chart_path}")

                # Contributions per member
                plt.figure(figsize=(12, 6))
                plt.barh(
                    top_teams.index,
                    top_teams["contributions_per_member"],
                    color="salmon",
                )
                plt.xlabel("Contributions per Member")
                plt.ylabel("Team")
                plt.title(f"Team Efficiency (Contributions per Member) - {org_name}")
                plt.tight_layout()

                # Save visualization
                chart_path = os.path.join(output_dir, f"{org_name}_team_efficiency.png")
                plt.savefig(chart_path)
                print(f"\nTeam efficiency chart saved to: {chart_path}")

    except Exception as e:
        logger.warning(f"Could not create visualizations: {str(e)}")
        print(
            "Note: Visualizations were not created. Make sure matplotlib is installed."
        )

    print(
        "\nAnalysis complete! Results have been saved to CSV files in the output directory."
    )
    print(f"Output directory: {output_dir}")


if __name__ == "__main__":
    main()

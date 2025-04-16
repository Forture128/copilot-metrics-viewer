"""
GitHub Team Analysis CLI

CLI tool for analyzing GitHub team and member data using the github_data_utils module.
"""

import os
import click
from datetime import datetime
from dotenv import load_dotenv

from src.utils.github_data_utils import (
    process_team_data,
    extract_member_contributions,
    extract_team_members,
    load_team_data_from_file,
    load_member_data_from_file,
    analyze_github_data,
    get_top_contributors,
    get_teams_by_activity,
    get_unique_members,
)
from src.utils.loggers import get_logger
from src.utils.helper_functions import ensure_output_dir
from airflow.models import Variable

logger = get_logger("github_team_analysis")

ANALYSIS_OUTPUT_DIR = Variable.get(
    "ANALYSIS_OUTPUT_DIR", "data/raw/github/github_teams"
)


@click.group()
def cli():
    """Command line interface for analyzing GitHub team and member data."""
    load_dotenv()  # Load environment variables from .env file
    pass


@cli.command()
@click.option(
    "--org",
    default="moneyforward",
    help="GitHub organization name",
)
@click.option(
    "--team-file",
    required=True,
    help="Path to the team data JSON file",
)
@click.option(
    "--output-dir",
    default=ANALYSIS_OUTPUT_DIR,
    help="Directory to save analysis results",
)
def analyze_teams(org, team_file, output_dir):
    """
    Analyze GitHub team data and generate summary statistics.

    Extracts team structure, members, and generates basic statistics.
    """
    logger.info(f"Analyzing team data for {org} from {team_file}")

    # Ensure output directory exists
    output_dir = ensure_output_dir(output_dir)

    # Process team data
    team_result = process_team_data(team_file)

    # Generate CSV of team members
    team_df = team_result["dataframe"]
    timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
    csv_path = os.path.join(output_dir, f"{org}_team_members_{timestamp}.csv")
    team_df.to_csv(csv_path, index=False)

    # Print summary
    click.echo(f"Organization: {team_result['organization']}")
    click.echo(f"Total teams: {team_result['total_teams']}")
    click.echo(f"Total unique members: {team_result['total_unique_members']}")
    click.echo(f"Team members saved to: {csv_path}")

    # Save unique members to file
    members_path = os.path.join(output_dir, f"{org}_unique_members_{timestamp}.txt")
    with open(members_path, "w") as f:
        for member in team_result["unique_members"]:
            f.write(f"{member}\n")

    click.echo(f"Unique members list saved to: {members_path}")


@cli.command()
@click.option(
    "--org",
    default="moneyforward",
    help="GitHub organization name",
)
@click.option(
    "--team-file",
    help="Path to the team data JSON file",
)
@click.option(
    "--member-file",
    help="Path to the member data JSON file",
)
@click.option(
    "--data-dir",
    default="data/github",
    help="Directory to search for data files if no specific paths provided",
)
@click.option(
    "--output-dir",
    default=ANALYSIS_OUTPUT_DIR,
    help="Directory to save analysis results",
)
def comprehensive_analysis(org, team_file, member_file, data_dir, output_dir):
    """
    Perform comprehensive analysis on GitHub team and member data.

    Combines team and member data to generate statistics and insights.
    """
    logger.info(f"Performing comprehensive analysis for {org}")

    # Ensure output directory exists
    output_dir = ensure_output_dir(output_dir)

    # Analyze GitHub data
    result = analyze_github_data(
        team_file_path=team_file,
        member_file_path=member_file,
        data_dir=data_dir,
        org_name=org,
        output_dir=output_dir,
    )

    # Print summary
    summary = result["summary"]
    click.echo(f"Organization: {summary['organization']}")
    click.echo(f"Team count: {summary['team_count']}")
    click.echo(f"Member count: {summary['member_count']}")
    click.echo(f"Unique member count: {summary['unique_member_count']}")

    if "total_commits" in summary:
        click.echo("\nContribution Summary:")
        click.echo(f"Total commits: {summary['total_commits']}")
        click.echo(f"Total pull requests: {summary['total_prs']}")
        click.echo(f"Total issues: {summary['total_issues']}")
        click.echo(f"Total reviews: {summary['total_reviews']}")

    # Generate output paths
    if "team_csv_path" in summary:
        click.echo(f"\nTeam data saved to: {summary['team_csv_path']}")

    if "member_csv_path" in summary:
        click.echo(f"Member data saved to: {summary['member_csv_path']}")

    if "combined_csv_path" in summary:
        click.echo(f"Combined data saved to: {summary['combined_csv_path']}")

    # Get top contributors
    if not result["member_df"].empty:
        click.echo("\nTop 5 Contributors:")
        top_contributors = get_top_contributors(result["member_df"], n=5)

        for _, contributor in top_contributors.iterrows():
            name = contributor.get("member_name") or contributor["member_login"]
            total = int(contributor.get("total_contributions", 0))
            click.echo(f"- {name}: {total} contributions")

    # Get team activity
    if not result["combined_df"].empty:
        click.echo("\nTop 5 Most Active Teams:")
        team_activity = get_teams_by_activity(result["combined_df"])

        for i, (team_name, row) in enumerate(team_activity.head(5).iterrows()):
            total = int(row["total_contributions"])
            members = int(row["member_count"])
            per_member = round(row["contributions_per_member"], 1)
            click.echo(
                f"- {team_name}: {total} contributions ({members} members, {per_member} per member)"
            )


@cli.command()
@click.option(
    "--member-file",
    required=True,
    help="Path to the member data JSON file",
)
@click.option(
    "--output-file",
    help="Path to output CSV file",
)
@click.option(
    "--output-dir",
    default=ANALYSIS_OUTPUT_DIR,
    help="Directory to save analysis results",
)
def extract_members(member_file, output_file, output_dir):
    """
    Extract member contributions from a member data file to CSV format.
    """
    logger.info(f"Extracting member contributions from {member_file}")

    # Ensure output directory exists
    output_dir = ensure_output_dir(output_dir)

    # Load member data
    member_data = load_member_data_from_file(member_file)

    # Extract contributions
    member_df = extract_member_contributions(member_data)

    # Determine output file if not provided
    if not output_file:
        timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
        output_file = os.path.join(output_dir, f"member_contributions_{timestamp}.csv")

    # Save to CSV
    member_df.to_csv(output_file, index=False)

    # Print summary
    click.echo(f"Extracted {len(member_df)} member contributions")
    click.echo(f"Data saved to: {output_file}")

    # Show preview
    if not member_df.empty:
        click.echo("\nPreview of extracted data:")
        preview_cols = [
            "member_login",
            "total_commits",
            "total_prs",
            "total_issues",
            "total_reviews",
        ]
        available_cols = [col for col in preview_cols if col in member_df.columns]
        click.echo(member_df[available_cols].head().to_string())


@cli.command(name="extract-team-members")
@click.option(
    "--team-file",
    required=True,
    help="Path to the team data JSON file",
)
@click.option(
    "--output-file",
    help="Path to output CSV file",
)
@click.option(
    "--list-unique",
    is_flag=True,
    help="List unique member logins",
)
@click.option(
    "--sort-by-teams",
    is_flag=True,
    help="Sort members by number of teams they belong to",
)
@click.option(
    "--output-dir",
    default=ANALYSIS_OUTPUT_DIR,
    help="Directory to save analysis results",
)
def extract_team_members_command(
    team_file, output_file, list_unique, sort_by_teams, output_dir
):
    """
    Extract team members from a team data file to CSV format.

    This command extracts the team-member relationships from a team data file
    and outputs them to a CSV file for further analysis.
    """
    logger.info(f"Extracting team members from {team_file}")

    # Ensure output directory exists
    output_dir = ensure_output_dir(output_dir)

    # Load team data
    team_data = load_team_data_from_file(team_file)

    # Extract team members to DataFrame
    members_df = extract_team_members(team_data)

    # Determine output file if not provided
    if not output_file:
        timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
        output_file = os.path.join(output_dir, f"team_members_{timestamp}.csv")

    # Save to CSV
    members_df.to_csv(output_file, index=False)

    # Print summary
    click.echo(f"Organization: {team_data.get('organization', 'unknown')}")
    click.echo(f"Extracted {len(members_df)} team-member relationships")
    click.echo(f"Total teams: {len(members_df['team_name'].unique())}")

    unique_members = get_unique_members(members_df)
    click.echo(f"Total unique members: {len(unique_members)}")
    click.echo(f"Data saved to: {output_file}")

    # Show unique members if requested
    if list_unique:
        click.echo("\nUnique member logins:")
        for member in unique_members:
            click.echo(f"- {member}")

    # Show members sorted by team count if requested
    if sort_by_teams:
        member_team_counts = (
            members_df.groupby("member_login").size().sort_values(ascending=False)
        )
        click.echo("\nMembers by team count:")
        for member, count in member_team_counts.head(10).items():
            click.echo(f"- {member}: {count} teams")

    # Show team member counts
    team_counts = members_df.groupby("team_name").size().sort_values(ascending=False)
    click.echo("\nTop 5 teams by member count:")
    for team, count in team_counts.head(5).items():
        click.echo(f"- {team}: {count} members")


def main():
    """CLI entry point"""
    cli()


if __name__ == "__main__":
    main()

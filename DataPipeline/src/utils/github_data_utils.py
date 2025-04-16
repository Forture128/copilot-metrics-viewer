"""
GitHub Data Utilities

This module provides helper functions for processing GitHub data using pandas.
It focuses on efficient data manipulation and analysis of team and member data.
"""

import pandas as pd
import json
from typing import List, Dict, Any, Optional, Tuple
import os
import logging
from pathlib import Path
from src.utils.helper_functions import ensure_output_dir

logger = logging.getLogger(__name__)


def load_team_data_from_file(team_file_path: str) -> Dict[str, Any]:
    """
    Load team data from a JSON file.

    Args:
        team_file_path: Path to the team JSON file

    Returns:
        Dictionary containing the team data
    """
    try:
        with open(team_file_path, "r") as f:
            team_data = json.load(f)
        return team_data
    except Exception as e:
        logger.error(f"Error loading team data from {team_file_path}: {str(e)}")
        raise


def load_team_members_from_csv(csv_file_path: str) -> List[str]:
    """
    Load team members from a CSV file.

    Args:
        csv_file_path: Path to the CSV file containing team member data

    Returns:
        List of unique member logins
    """
    try:
        # Read CSV file into DataFrame
        df = pd.read_csv(csv_file_path)

        # Check if 'member_login' column exists
        if "member_login" not in df.columns:
            logger.error(
                f"CSV file {csv_file_path} does not contain 'member_login' column"
            )
            return []

        # Get unique member logins
        unique_members = df["member_login"].unique().tolist()
        logger.info(f"Loaded {len(unique_members)} unique members from {csv_file_path}")
        return unique_members
    except Exception as e:
        logger.error(f"Error loading team members from CSV {csv_file_path}: {str(e)}")
        return []


def extract_team_members(team_data: Dict[str, Any]) -> pd.DataFrame:
    """
    Extract team-member relationships from team data into a pandas DataFrame.

    Args:
        team_data: Dictionary containing team data

    Returns:
        DataFrame with team and member information
    """
    team_members = []

    for team in team_data.get("teams", []):
        team_name = team.get("name")
        team_slug = team.get("slug")
        team_id = team.get("id")
        team_description = team.get("description")
        team_privacy = team.get("privacy")

        # Get repository information
        repos = []
        if "repositories" in team and "nodes" in team["repositories"]:
            repos = [
                repo.get("name") if repo else "unknown"
                for repo in team["repositories"]["nodes"]
            ]

        # Get members
        members_data = team.get("members", {})
        members_count = members_data.get("totalCount", 0)
        members_nodes = members_data.get("nodes", [])

        for member in members_nodes:
            if "login" in member:
                team_members.append(
                    {
                        "team_id": team_id,
                        "team_name": team_name,
                        "team_slug": team_slug,
                        "team_description": team_description,
                        "team_privacy": team_privacy,
                        "team_repo_count": len(repos),
                        "team_member_count": members_count,
                        "member_login": member["login"],
                        "member_name": member.get("name"),
                        "member_email": member.get("email"),
                        "member_company": member.get("company"),
                        "member_location": member.get("location"),
                    }
                )

    # Convert to DataFrame
    if team_members:
        df = pd.DataFrame(team_members)
        return df
    else:
        # Return empty DataFrame with expected columns
        return pd.DataFrame(
            columns=[
                "team_id",
                "team_name",
                "team_slug",
                "team_description",
                "team_privacy",
                "team_repo_count",
                "team_member_count",
                "member_login",
                "member_name",
                "member_email",
                "member_company",
                "member_location",
            ]
        )


def get_unique_members(team_members_df: pd.DataFrame) -> List[str]:
    """
    Get a list of unique member logins from the team members DataFrame.

    Args:
        team_members_df: DataFrame with team member information

    Returns:
        List of unique member logins
    """
    if team_members_df.empty:
        return []
    return team_members_df["member_login"].unique().tolist()


def process_team_data(team_file_path: str) -> Dict[str, Any]:
    """
    Process team data to extract useful statistics and unique members.

    Args:
        team_file_path: Path to the team JSON file

    Returns:
        Dictionary with team statistics and unique members
    """
    # Load team data
    team_data = load_team_data_from_file(team_file_path)

    # Extract team-member relationships into DataFrame
    team_members_df = extract_team_members(team_data)

    # Get unique members
    unique_members = get_unique_members(team_members_df)

    # Calculate statistics
    if not team_members_df.empty:
        team_counts = team_members_df.groupby("team_name").size().to_dict()
        member_team_counts = team_members_df.groupby("member_login").size().to_dict()

        result = {
            "organization": team_data.get("organization", ""),
            "collected_at": team_data.get("collected_at", ""),
            "unique_members": unique_members,
            "team_counts": team_counts,  # Members per team
            "member_team_counts": member_team_counts,  # Teams per member
            "total_teams": len(team_members_df["team_name"].unique()),
            "total_unique_members": len(unique_members),
            "dataframe": team_members_df,  # Keep DataFrame for further analysis
        }
    else:
        result = {
            "organization": team_data.get("organization", ""),
            "collected_at": team_data.get("collected_at", ""),
            "unique_members": [],
            "team_counts": {},
            "member_team_counts": {},
            "total_teams": 0,
            "total_unique_members": 0,
            "dataframe": team_members_df,
        }

    return result


def extract_members_from_team_json(team_file_path: str) -> List[str]:
    """
    Extract unique member logins from a team JSON file.

    Args:
        team_file_path: Path to the team JSON file

    Returns:
        List of unique member logins
    """
    try:
        with open(team_file_path, "r") as f:
            team_data = json.load(f)

        # Extract member logins from team data
        members = list(
            {
                m["login"]
                for t in team_data.get("teams", [])
                for m in t.get("members", {}).get("nodes", [])
                if "login" in m
            }
        )

        logger.info(f"Extracted {len(members)} unique members from {team_file_path}")
        return members
    except Exception as e:
        logger.error(
            f"Error extracting members from team JSON {team_file_path}: {str(e)}"
        )
        return []


def load_member_data_from_file(member_file_path: str) -> Dict[str, Any]:
    """
    Load member data from a JSON file.

    Args:
        member_file_path: Path to the member JSON file

    Returns:
        Dictionary containing the member data
    """
    try:
        with open(member_file_path, "r") as f:
            member_data = json.load(f)
        return member_data
    except Exception as e:
        logger.error(f"Error loading member data from {member_file_path}: {str(e)}")
        raise


def extract_member_contributions(member_data: Dict[str, Any]) -> pd.DataFrame:
    """
    Extract member contributions from member data into a pandas DataFrame.

    Args:
        member_data: Dictionary containing member data

    Returns:
        DataFrame with member contribution information
    """
    contributions = []

    for member in member_data.get("members", []):
        member_login = member.get("login")
        if not member_login:
            continue

        contrib_data = {
            "member_login": member_login,
            "member_name": member.get("name"),
            "member_email": member.get("email"),
            "member_company": member.get("company"),
            "member_location": member.get("location"),
            "member_created_at": member.get("createdAt"),
            "member_updated_at": member.get("updatedAt"),
            "total_commits": 0,
            "total_issues": 0,
            "total_prs": 0,
            "total_reviews": 0,
        }

        # Extract contribution metrics if available
        if "contributionsCollection" in member:
            contrib = member["contributionsCollection"]
            contrib_data.update(
                {
                    "total_commits": contrib.get("totalCommitContributions", 0),
                    "total_issues": contrib.get("totalIssueContributions", 0),
                    "total_prs": contrib.get("totalPullRequestContributions", 0),
                    "total_reviews": contrib.get(
                        "totalPullRequestReviewContributions", 0
                    ),
                }
            )

        contributions.append(contrib_data)

    # Convert to DataFrame
    if contributions:
        df = pd.DataFrame(contributions)
        return df
    else:
        # Return empty DataFrame with expected columns
        return pd.DataFrame(
            columns=[
                "member_login",
                "member_name",
                "member_email",
                "member_company",
                "member_location",
                "member_created_at",
                "member_updated_at",
                "total_commits",
                "total_issues",
                "total_prs",
                "total_reviews",
            ]
        )


def combine_team_and_member_data(
    team_df: pd.DataFrame, member_df: pd.DataFrame
) -> pd.DataFrame:
    """
    Combine team membership data with member contribution data.

    Args:
        team_df: DataFrame with team-member relationships
        member_df: DataFrame with member contribution data

    Returns:
        Combined DataFrame with team and contribution information
    """
    if team_df.empty or member_df.empty:
        return pd.DataFrame()

    # Merge DataFrames on member_login
    combined_df = pd.merge(
        team_df, member_df, on="member_login", how="left", suffixes=("_team", "_member")
    )

    # Fill missing values
    for col in ["total_commits", "total_issues", "total_prs", "total_reviews"]:
        if col in combined_df.columns:
            combined_df[col] = combined_df[col].fillna(0)

    return combined_df


def find_latest_data_files(
    data_dir: str, org_name: str = None
) -> Tuple[Optional[str], Optional[str]]:
    """
    Find the latest team and member data files in the data directory.

    Args:
        data_dir: Directory containing GitHub data files
        org_name: Optional organization name to filter files

    Returns:
        Tuple containing (latest_team_file_path, latest_member_file_path)
    """
    data_path = Path(data_dir)

    # Prepare search patterns
    team_pattern = (
        f"github_teams_{org_name}_*.json" if org_name else "github_teams_*.json"
    )
    member_pattern = (
        f"github_members_{org_name}_*.json" if org_name else "github_members_*.json"
    )

    # Find matching files
    team_files = sorted(
        data_path.glob(team_pattern), key=os.path.getmtime, reverse=True
    )
    member_files = sorted(
        data_path.glob(member_pattern), key=os.path.getmtime, reverse=True
    )

    latest_team_file = str(team_files[0]) if team_files else None
    latest_member_file = str(member_files[0]) if member_files else None

    return latest_team_file, latest_member_file


def find_latest_team_member_csv(data_dir: str, org_name: str = None) -> Optional[str]:
    """
    Find the latest team members CSV file in the data directory.

    Args:
        data_dir: Directory containing GitHub data files
        org_name: Optional organization name to filter files

    Returns:
        Path to the latest team members CSV file, or None if not found
    """
    data_path = Path(data_dir)

    # Prepare search pattern - look for both standard naming and analysis output
    patterns = []
    if org_name:
        patterns.extend(
            [f"{org_name}_team_members_*.csv", f"team_members_{org_name}_*.csv"]
        )
    else:
        patterns.extend(["*team_members_*.csv"])

    # Find all matching files
    all_csv_files = []
    for pattern in patterns:
        all_csv_files.extend(data_path.glob(pattern))

    # Sort by modification time
    sorted_files = sorted(all_csv_files, key=os.path.getmtime, reverse=True)

    return str(sorted_files[0]) if sorted_files else None


def analyze_github_data(
    team_file_path: Optional[str] = None,
    member_file_path: Optional[str] = None,
    data_dir: Optional[str] = None,
    org_name: Optional[str] = None,
    output_dir: Optional[str] = None,
) -> Dict[str, Any]:
    """
    Analyze GitHub team and member data and generate summary statistics.

    Args:
        team_file_path: Path to team data JSON file (optional)
        member_file_path: Path to member data JSON file (optional)
        data_dir: Directory to search for data files if no specific paths provided
        org_name: Organization name for filtering files
        output_dir: Directory to save output files (optional)

    Returns:
        Dictionary with analysis results
    """
    # If file paths not provided, try to find latest files
    if (not team_file_path or not member_file_path) and data_dir:
        latest_team_file, latest_member_file = find_latest_data_files(
            data_dir, org_name
        )
        team_file_path = team_file_path or latest_team_file
        member_file_path = member_file_path or latest_member_file

    # Process team data if available
    team_result = None
    team_df = pd.DataFrame()
    if team_file_path:
        try:
            team_result = process_team_data(team_file_path)
            team_df = team_result["dataframe"]
            logger.info(
                f"Processed team data: {len(team_df)} team-member relationships"
            )
        except Exception as e:
            logger.error(f"Error processing team data: {str(e)}")

    # Process member data if available
    member_df = pd.DataFrame()
    if member_file_path:
        try:
            member_data = load_member_data_from_file(member_file_path)
            member_df = extract_member_contributions(member_data)
            logger.info(f"Processed member data: {len(member_df)} members")
        except Exception as e:
            logger.error(f"Error processing member data: {str(e)}")

    # Combine data if both available
    combined_df = pd.DataFrame()
    if not team_df.empty and not member_df.empty:
        combined_df = combine_team_and_member_data(team_df, member_df)
        logger.info(f"Combined data: {len(combined_df)} rows")

    # Calculate summary statistics
    summary = {
        "organization": org_name
        or (team_result.get("organization") if team_result else ""),
        "team_count": len(team_df["team_name"].unique()) if not team_df.empty else 0,
        "member_count": len(member_df) if not member_df.empty else 0,
        "unique_member_count": len(team_df["member_login"].unique())
        if not team_df.empty
        else 0,
    }

    # Add contribution stats if available
    if not member_df.empty and "total_commits" in member_df.columns:
        summary.update(
            {
                "total_commits": member_df["total_commits"].sum(),
                "total_issues": member_df["total_issues"].sum(),
                "total_prs": member_df["total_prs"].sum(),
                "total_reviews": member_df["total_reviews"].sum(),
            }
        )

    # Save output files if directory provided
    if output_dir and (not team_df.empty or not member_df.empty):
        # Ensure the output directory exists
        output_dir = ensure_output_dir(output_dir)

        timestamp = pd.Timestamp.now().strftime("%Y%m%d_%H%M%S")
        org_prefix = f"{org_name}_" if org_name else ""

        # Save team data
        if not team_df.empty:
            team_csv = os.path.join(
                output_dir, f"{org_prefix}team_members_{timestamp}.csv"
            )
            team_df.to_csv(team_csv, index=False)
            summary["team_csv_path"] = str(team_csv)

        # Save member data
        if not member_df.empty:
            member_csv = os.path.join(
                output_dir, f"{org_prefix}member_contributions_{timestamp}.csv"
            )
            member_df.to_csv(member_csv, index=False)
            summary["member_csv_path"] = str(member_csv)

        # Save combined data
        if not combined_df.empty:
            combined_csv = os.path.join(
                output_dir, f"{org_prefix}combined_data_{timestamp}.csv"
            )
            combined_df.to_csv(combined_csv, index=False)
            summary["combined_csv_path"] = str(combined_csv)

    # Store DataFrames in result for further use
    result = {
        "summary": summary,
        "team_df": team_df,
        "member_df": member_df,
        "combined_df": combined_df,
    }

    return result


def get_top_contributors(member_df: pd.DataFrame, n: int = 10) -> pd.DataFrame:
    """
    Get the top N contributors based on total contributions.

    Args:
        member_df: DataFrame with member contribution data
        n: Number of top contributors to return

    Returns:
        DataFrame with top contributors
    """
    if member_df.empty:
        return pd.DataFrame()

    # Calculate total contributions
    if "total_contributions" not in member_df.columns:
        member_df = member_df.copy()
        contribution_cols = [
            "total_commits",
            "total_issues",
            "total_prs",
            "total_reviews",
        ]
        cols_to_sum = [col for col in contribution_cols if col in member_df.columns]

        if cols_to_sum:
            member_df["total_contributions"] = member_df[cols_to_sum].sum(axis=1)
        else:
            return pd.DataFrame()

    # Sort and return top N
    return member_df.sort_values("total_contributions", ascending=False).head(n)


def get_teams_by_activity(combined_df: pd.DataFrame) -> pd.DataFrame:
    """
    Get teams ranked by contribution activity.

    Args:
        combined_df: Combined DataFrame with team and member contribution data

    Returns:
        DataFrame with teams ranked by activity
    """
    if combined_df.empty:
        return pd.DataFrame()

    # Sum contributions by team
    contribution_cols = ["total_commits", "total_issues", "total_prs", "total_reviews"]
    cols_to_sum = [col for col in contribution_cols if col in combined_df.columns]

    if not cols_to_sum:
        return pd.DataFrame()

    # Group by team and sum contributions
    team_activity = combined_df.groupby("team_name")[cols_to_sum].sum()
    team_activity["total_contributions"] = team_activity[cols_to_sum].sum(axis=1)
    team_activity["member_count"] = combined_df.groupby("team_name")[
        "member_login"
    ].nunique()

    # Calculate per-member metrics
    team_activity["contributions_per_member"] = (
        team_activity["total_contributions"] / team_activity["member_count"]
    )

    return team_activity.sort_values("total_contributions", ascending=False)

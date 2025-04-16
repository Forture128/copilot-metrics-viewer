# GitHub CLI Tools

This directory contains command-line interfaces for GitHub data collection and analysis.

## Structure

```
github/
├── collector_cli.py  # GitHub data collection commands
├── team_analysis.py  # GitHub team and member analysis commands
└── README.md         # This file
```

## Collector CLI

The collector CLI provides commands for collecting GitHub data:

```bash
# Collect repository data
python -m src.cli.github.collector_cli collect-repository --org your-org-name

# Collect with Redis caching
python -m src.cli.github.collector_cli collect-repository --org your-org-name --use-redis

# Collect team data
python -m src.cli.github.collector_cli collect-teams --org your-org-name

# Collect member data for all organization members
python -m src.cli.github.collector_cli collect-members --org your-org-name --detailed-info

# Collect contributions for specific members
python -m src.cli.github.collector_cli collect-members-contributions --org your-org-name --team-csv team_members.csv
```

For more options and details, run:

```bash
python -m src.cli.github.collector_cli --help
python -m src.cli.github.collector_cli collect-repository --help
python -m src.cli.github.collector_cli collect-teams --help
python -m src.cli.github.collector_cli collect-members --help
python -m src.cli.github.collector_cli collect-members-contributions --help
```

### Member Data Collection Options

The GitHub collector now provides two ways to collect member contribution data:

1. `collect-members`: Collects full profile and contribution data for all organization members.
2. `collect-members-contributions`: Focuses only on contribution data for a specific subset of members.

#### Collecting Data for Specific Members

You can specify which members to collect data for in several ways:

```bash
# From a CSV file with a member_login column
python -m src.cli.github.collector_cli collect-members-contributions --org your-org-name --team-csv team_members.csv

# From a team JSON file (extracts all member logins)
python -m src.cli.github.collector_cli collect-members-contributions --org your-org-name --team-file data/github/github_teams_your-org-name_20230101_120000.json

# It will also look for the latest team members CSV if no source is specified
python -m src.cli.github.collector_cli collect-members-contributions --org your-org-name
```

## Rate Limit Improvements

The collector now includes smart rate limit management to minimize unnecessary API calls:

- Checks rate limits periodically (every 10 batches) instead of after every batch
- Increases check frequency when approaching the limit
- Uses a progressive wait strategy for rate limit resets
- Caches rate limit information to reduce API calls

You can configure rate limit behavior using environment variables:

```bash
# Set rate limit buffer (reserve)
export GITHUB_RATE_LIMIT_BUFFER=100

# Set minimum time between rate limit checks (seconds)
export GITHUB_RATE_LIMIT_CHECK_INTERVAL=60
```

## Team Analysis CLI

The team analysis CLI provides commands for analyzing GitHub team and member data:

```bash
# Analyze team data
python -m src.cli.github.team_analysis analyze-teams --org your-org-name --team-file data/github/github_teams_your-org-name_20230101_120000.json

# Perform comprehensive analysis of team and member data
python -m src.cli.github.team_analysis comprehensive-analysis --org your-org-name --team-file data/github/github_teams_your-org-name_20230101_120000.json --member-file data/github/github_members_your-org-name_20230101_120000.json

# Extract member contributions to CSV
python -m src.cli.github.team_analysis extract-members --member-file data/github/github_members_your-org-name_20230101_120000.json

# Extract team members to CSV
python -m src.cli.github.team_analysis extract-team-members --team-file data/github/github_teams_your-org-name_20230101_120000.json
```

For more options and details, run:

```bash
python -m src.cli.github.team_analysis --help
python -m src.cli.github.team_analysis analyze-teams --help
python -m src.cli.github.team_analysis comprehensive-analysis --help
python -m src.cli.github.team_analysis extract-members --help
python -m src.cli.github.team_analysis extract-team-members --help
```

## Member Contributions Analysis

The GitHub data utilities provide powerful capabilities for analyzing member contributions using pandas. Here's how to use these utilities in your workflows:

### Collecting Member Contributions

First, collect member data using the collector CLI:

```bash
# Collect basic member data
python -m src.cli.github.collector_cli collect-members --org your-org-name

# Collect detailed member data with contributions
python -m src.cli.github.collector_cli collect-members --org your-org-name --detailed-info

# Collect contributions only for specific members
python -m src.cli.github.collector_cli collect-members-contributions --org your-org-name --team-csv team_members.csv
```

### Extracting and Processing Member Contributions

Use the team analysis CLI to extract contributions to CSV:

```bash
# Extract member contributions to CSV
python -m src.cli.github.team_analysis extract-members --member-file data/github/github_members_your-org-name_20230101_120000.json

# Extract member contributions from the specialized format
python -m src.cli.github.team_analysis extract-members --member-contributions-file data/github/github_members_contributions_your-org-name_20230101_120000.json
```

### Extracting Team Members

To extract team members using the CLI:

```bash
# Extract team members to CSV
python -m src.cli.github.team_analysis extract-team-members --team-file data/github/github_teams_your-org-name_20230101_120000.json

# List unique members
python -m src.cli.github.team_analysis extract-team-members --team-file data/github/github_teams_your-org-name_20230101_120000.json --list-unique

# Show members sorted by number of teams
python -m src.cli.github.team_analysis extract-team-members --team-file data/github/github_teams_your-org-name_20230101_120000.json --sort-by-teams
```

You can also use the GitHub data utilities directly in your Python scripts:

```python
from src.utils.github_data_utils import load_team_data_from_file, extract_team_members, get_unique_members

# Load team data
team_file = "data/github/github_teams_your-org-name_20230101_120000.json"
team_data = load_team_data_from_file(team_file)

# Extract team members to a DataFrame
team_members_df = extract_team_members(team_data)

# Get unique member logins
unique_members = get_unique_members(team_members_df)
print(f"Found {len(unique_members)} unique members")

# You can also get counts of members per team
team_counts = team_members_df.groupby("team_name").size().to_dict()
for team_name, count in team_counts.items():
    print(f"{team_name}: {count} members")

# Or teams per member
member_team_counts = team_members_df.groupby("member_login").size().to_dict()
print("\nMembers with most teams:")
for member, count in sorted(member_team_counts.items(), key=lambda x: x[1], reverse=True)[:5]:
    print(f"{member}: {count} teams")

# Save to CSV for further analysis
team_members_df.to_csv("team_members.csv", index=False)
```

### Using the Pandas Utilities in Python

You can also use the pandas utilities directly in your Python scripts:

```python
import pandas as pd
from src.utils.github_data_utils import (
    load_member_data_from_file,
    load_member_contributions_from_file,
    extract_member_contributions,
    get_top_contributors
)

# Option 1: Load and process full member data
member_file = "data/github/github_members_your-org-name_20230101_120000.json"
member_data = load_member_data_from_file(member_file)
member_df = extract_member_contributions(member_data)

# Option 2: Load and process member contributions data
contributions_file = "data/github/github_members_contributions_your-org-name_20230101_120000.json"
contrib_data = load_member_contributions_from_file(contributions_file)
contrib_df = extract_member_contributions(contrib_data, is_contributions_format=True)

# Get top contributors
top_contributors = get_top_contributors(member_df, n=10)
print("Top 10 contributors:")
for _, contributor in top_contributors.iterrows():
    name = contributor.get("member_name") or contributor["member_login"]
    total = int(contributor.get("total_contributions", 0))
    print(f"- {name}: {total} contributions")

# Save to CSV
member_df.to_csv("member_contributions.csv", index=False)
```

### Combining Team and Member Data

To get a comprehensive view of both team structure and member contributions:

```python
from src.utils.github_data_utils import (
    process_team_data,
    load_member_data_from_file,
    load_member_contributions_from_file,
    extract_member_contributions,
    combine_team_and_member_data,
    get_teams_by_activity
)

# Load team data
team_file = "data/github/github_teams_your-org-name_20230101_120000.json"
team_result = process_team_data(team_file)
team_df = team_result["dataframe"]

# Option 1: Load full member data
member_file = "data/github/github_members_your-org-name_20230101_120000.json"
member_data = load_member_data_from_file(member_file)
member_df = extract_member_contributions(member_data)

# Option 2: Load member contributions data
contributions_file = "data/github/github_members_contributions_your-org-name_20230101_120000.json"
contrib_data = load_member_contributions_from_file(contributions_file)
contrib_df = extract_member_contributions(contrib_data, is_contributions_format=True)

# Combine team and member data
combined_df = combine_team_and_member_data(team_df, member_df)

# Get teams ranked by activity
team_activity = get_teams_by_activity(combined_df)
print("Teams ranked by total contributions:")
for team_name, row in team_activity.head().iterrows():
    print(f"- {team_name}: {int(row['total_contributions'])} contributions")
```

### Example Analysis Script

For a complete example of how to use these utilities, see `scripts/github_data_analysis_example.py`. This script demonstrates:

- Finding latest data files automatically
- Processing team and member data
- Generating summary statistics
- Creating visualizations of top contributors and team activity
- Exporting data to CSV files for further analysis

Run the example script:

```bash
# Make sure you're in the DataPipeline directory
python scripts/github_data_analysis_example.py
```

## Configuration

CLI tools can be configured using:

1. Command line arguments
2. Environment variables
3. Configuration files

### Environment Variables

- `GITHUB_TOKEN`: GitHub Personal Access Token
- `USE_REDIS`: Enable Redis caching (true/false)
- `REDIS_HOST`: Redis host (default: localhost)
- `REDIS_PORT`: Redis port (default: 6379)
- `REDIS_DB`: Redis database number (default: 0)
- `GITHUB_RATE_LIMIT_BUFFER`: Number of API calls to keep in reserve (default: 100)
- `GITHUB_RATE_LIMIT_CHECK_INTERVAL`: Seconds between rate limit checks (default: 60)

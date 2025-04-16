# GitHub Team and Member Data Guide

This document explains the structure and content of the GitHub team and member data collected by the GitHub Collector CLI.

## Overview

The GitHub Collector CLI provides separate commands to collect team and member data:

- `collect_teams`: Collects team structure, members, and repository assignments
- `collect_members`: Collects detailed user information and contribution metrics for all organization members
- `collect_members_contributions`: Collects contributions for a specific subset of members
  - Both commands support reading member logins from CSV files or team JSON files

## Team Data Structure

Team data is stored in a JSON file with the following structure:

```json
{
  "organization": "moneyforward",
  "collected_at": "2025-04-06T12:34:56.789012",
  "teams": [
    {
      "id": "MDQ6VGVhbTEyMzQ1Ng==",
      "name": "Engineering",
      "slug": "engineering",
      "description": "Engineering team",
      "createdAt": "2023-01-01T00:00:00Z",
      "updatedAt": "2024-01-01T00:00:00Z",
      "privacy": "VISIBLE",
      "repositories": {
        "totalCount": 10,
        "nodes": [
          {
            "name": "repo1",
            "url": "https://github.com/moneyforward/repo1"
          }
        ]
      },
      "members": {
        "totalCount": 5,
        "nodes": [
          {
            "login": "user1",
            "name": "User One",
            "email": "user1@example.com",
            "company": "MoneyForward",
            "createdAt": "2020-01-01T00:00:00Z",
            "updatedAt": "2024-01-01T00:00:00Z",
            "location": "Tokyo, Japan",
            "isHireable": false,
            "isSiteAdmin": false,
            "organizationVerifiedDomainEmails": ["user1@moneyforward.com"]
          }
        ]
      },
      "childTeams": {
        "totalCount": 2,
        "nodes": [
          {
            "name": "Frontend",
            "slug": "frontend"
          }
        ]
      },
      "parentTeam": {
        "name": "Product",
        "slug": "product"
      }
    }
  ],
  "metadata": {
    "team_count": 15,
    "include_members": true
  }
}
```

### Key Team Data Fields

- `id`: GitHub's internal team ID
- `name`: Display name of the team
- `slug`: URL-friendly name used in GitHub URLs
- `privacy`: Team visibility (VISIBLE, SECRET)
- `repositories`: List of repositories assigned to the team
- `members`: List of team members (if `include_members` is true)
- `childTeams`: Sub-teams within this team
- `parentTeam`: Parent team if this is a sub-team

## Member Data Structure

Member data is stored in a JSON file with the following structure:

```json
{
  "organization": "moneyforward",
  "collected_at": "2025-04-06T12:34:56.789012",
  "members": [
    {
      "login": "user1",
      "name": "User One",
      "email": "user1@example.com",
      "company": "MoneyForward",
      "createdAt": "2020-01-01T00:00:00Z",
      "updatedAt": "2024-01-01T00:00:00Z",
      "location": "Tokyo, Japan",
      "isHireable": false,
      "isSiteAdmin": false,
      "organizationVerifiedDomainEmails": ["user1@moneyforward.com"],
      "contributionsCollection": {
        "totalCommitContributions": 250,
        "totalIssueContributions": 50,
        "totalPullRequestContributions": 125,
        "totalPullRequestReviewContributions": 75,
        "commitContributionsByRepository": [
          {
            "repository": {
              "name": "repo1"
            },
            "contributions": {
              "totalCount": 150
            }
          }
        ],
        "issueContributionsByRepository": [
          {
            "repository": {
              "name": "repo1"
            },
            "contributions": {
              "totalCount": 30
            }
          }
        ],
        "pullRequestContributionsByRepository": [
          {
            "repository": {
              "name": "repo1"
            },
            "contributions": {
              "totalCount": 75
            }
          }
        ]
      }
    }
  ],
  "metadata": {
    "member_count": 50,
    "include_detailed_info": true
  }
}
```

### Key Member Data Fields

- `login`: GitHub username
- `name`: Display name
- `email`: Public email address (may be null)
- `organizationVerifiedDomainEmails`: Email addresses verified with the organization's domain
- `contributionsCollection`: Only present if `detailed_info` is true
  - `totalCommitContributions`: Total number of commits
  - `totalIssueContributions`: Total number of issues opened
  - `totalPullRequestContributions`: Total number of PRs opened
  - `totalPullRequestReviewContributions`: Total number of PR reviews
  - `commitContributionsByRepository`: Commit counts by repository
  - `issueContributionsByRepository`: Issue counts by repository
  - `pullRequestContributionsByRepository`: PR counts by repository

## Member Contributions Data Structure

When using the `collect_members_contributions` command, the data is stored in a slightly different format:

```json
{
  "organization": "moneyforward",
  "collected_at": "2025-04-06T12:34:56.789012",
  "members_contributions": {
    "user1": {
      "totalCommitContributions": 250,
      "totalIssueContributions": 50,
      "totalPullRequestContributions": 125,
      "totalPullRequestReviewContributions": 75,
      "commitContributionsByRepository": [
        {
          "repository": {
            "name": "repo1"
          },
          "contributions": {
            "totalCount": 150
          }
        }
      ],
      "issueContributionsByRepository": [
        {
          "repository": {
            "name": "repo1"
          },
          "contributions": {
            "totalCount": 30
          }
        }
      ],
      "pullRequestContributionsByRepository": [
        {
          "repository": {
            "name": "repo1"
          },
          "contributions": {
            "totalCount": 75
          }
        }
      ]
    },
    "user2": {
      "totalCommitContributions": 120,
      "totalIssueContributions": 25,
      "totalPullRequestContributions": 60,
      "totalPullRequestReviewContributions": 40,
      "commitContributionsByRepository": [
        // Repository-specific contributions
      ]
    }
  },
  "metadata": {
    "member_count": 2
  }
}
```

This format focuses exclusively on contributions data with a dictionary keyed by member login, making it efficient for tracking contributions for specific team members without collecting their full profile information.

## CSV File Support

The collector now supports extracting member logins from CSV files, making it easier to collect data for specific members.

### CSV File Format

The CSV file should contain a column named `member_login` with GitHub usernames:

```csv
member_login,name,email,team
user1,User One,user1@example.com,Engineering
user2,User Two,user2@example.com,Product
user3,User Three,user3@example.com,Design
```

Only the `member_login` column is required. Other columns are ignored.

### Using CSV Files with the CLI

You can use the `--team-csv` option to specify a CSV file:

```bash
# Collect data for members in the CSV file with collect-members
python -m src.cli.github.collector_cli collect-members moneyforward --team-csv data/github/team_members.csv --detailed-info

# Collect just contributions data for members in the CSV file
python -m src.cli.github.collector_cli collect-members-contributions moneyforward --team-csv data/github/team_members.csv
```

The collector can also automatically find and use the latest team members CSV file if one exists:

```bash
# The collector will look for the latest CSV file with the pattern:
# data/github/github_team_members_*.csv
python -m src.cli.github.collector_cli collect-members-contributions moneyforward
```

### Extracting Members from Team Data

You can also use a team JSON file to extract member logins:

```bash
# Collect contributions for members in a specific team file
python -m src.cli.github.collector_cli collect-members-contributions moneyforward --team-file data/github/github_teams_moneyforward_20250406_123456.json
```

This is useful when you want to collect contributions data for all members of certain teams without having to manually create a CSV file.

### Generating Member CSV Files

You can extract members from team data into a CSV file using utilities in the `github_data_utils` module:

```python
import pandas as pd
from src.utils.github_data_utils import extract_members_from_team_json

# Load team members from a JSON file into a CSV
team_file = "data/github/github_teams_moneyforward_20250406_123456.json"
members = extract_members_from_team_json(team_file)

# Create a DataFrame and save to CSV
df = pd.DataFrame({"member_login": members})
df.to_csv("data/github/team_members.csv", index=False)
```

## Choosing Between collect-members and collect-members-contributions

- Use `collect-members` when:

  - You need complete profile information for all organization members
  - You want to collect data for all members at once
  - You need a comprehensive dataset with profile details and contributions

- Use `collect-members-contributions` when:
  - You only need contribution statistics for specific members
  - You want to collect data for a subset of members (e.g., a specific team)
  - You want to optimize API call usage by only fetching contribution data
  - You're doing team-specific contribution analysis

## Data Analysis Examples

### Team Structure Analysis

Here are some common analyses you can perform with team data:

1. **Team Hierarchy Map**

   Create a tree diagram of team relationships using parent/child team data.

2. **Repository to Team Mapping**

   Analyze which teams are responsible for which repositories.

3. **Member Distribution**

   Analyze how members are distributed across different teams.

```python
import json
import pandas as pd
import matplotlib.pyplot as plt

# Load team data
with open('github_teams_moneyforward_20250406_123456.json') as f:
    team_data = json.load(f)

# Extract teams and their member counts
teams = []
for team in team_data['teams']:
    teams.append({
        'name': team['name'],
        'member_count': team['members']['totalCount'],
        'repo_count': team['repositories']['totalCount']
    })

# Create a DataFrame and visualize
df = pd.DataFrame(teams)
df.plot(x='name', y='member_count', kind='bar', title='Team Member Distribution')
plt.savefig('team_member_distribution.png')
```

### Member Contribution Analysis

Here are some common analyses you can perform with member data:

1. **Top Contributors**

   Identify members with the highest contribution counts.

2. **Contribution Type Distribution**

   Analyze what types of contributions members make (commits, PRs, issues, reviews).

3. **Repository Activity**

   Identify which repositories have the most activity.

```python
import json
import pandas as pd
import matplotlib.pyplot as plt

# Load member data
with open('github_members_moneyforward_20250406_123456.json') as f:
    member_data = json.load(f)

# Extract member contribution data
members = []
for member in member_data['members']:
    if 'contributionsCollection' in member:
        contrib = member['contributionsCollection']
        members.append({
            'login': member['login'],
            'name': member['name'] or member['login'],
            'commits': contrib['totalCommitContributions'],
            'pull_requests': contrib['totalPullRequestContributions'],
            'issues': contrib['totalIssueContributions'],
            'reviews': contrib['totalPullRequestReviewContributions'],
            'total': (
                contrib['totalCommitContributions'] +
                contrib['totalPullRequestContributions'] +
                contrib['totalIssueContributions'] +
                contrib['totalPullRequestReviewContributions']
            )
        })

# Create a DataFrame and visualize
df = pd.DataFrame(members)
df = df.sort_values('total', ascending=False).head(10)
df.plot(x='login', y=['commits', 'pull_requests', 'issues', 'reviews'],
        kind='bar', stacked=True, title='Top 10 Contributors by Type')
plt.savefig('top_contributors.png')
```

### Analysis with Member Contributions Data

If you've collected member contributions using the `collect-members-contributions` command:

```python
import json
import pandas as pd
import matplotlib.pyplot as plt

# Load contributions data
with open('github_members_contributions_moneyforward_20250406_123456.json') as f:
    contrib_data = json.load(f)

# Extract contribution data
members = []
for login, contrib in contrib_data['members_contributions'].items():
    members.append({
        'login': login,
        'commits': contrib['totalCommitContributions'],
        'pull_requests': contrib['totalPullRequestContributions'],
        'issues': contrib['totalIssueContributions'],
        'reviews': contrib['totalPullRequestReviewContributions'],
        'total': (
            contrib['totalCommitContributions'] +
            contrib['totalPullRequestContributions'] +
            contrib['totalIssueContributions'] +
            contrib['totalPullRequestReviewContributions']
        )
    })

# Create a DataFrame and analyze
df = pd.DataFrame(members)
print(f"Total members: {len(df)}")
print(f"Total contributions: {df['total'].sum()}")
print("\nTop 5 contributors:")
for _, row in df.sort_values('total', ascending=False).head(5).iterrows():
    print(f"- {row['login']}: {row['total']} contributions")
```

## Combining Team and Member Data

To get a comprehensive view, you can combine team and member data:

```python
import json
import pandas as pd

# Load team and member data
with open('github_teams_moneyforward_20250406_123456.json') as f:
    team_data = json.load(f)

with open('github_members_moneyforward_20250406_123456.json') as f:
    member_data = json.load(f)

# Create a dictionary of members with their contribution counts
member_dict = {}
for member in member_data['members']:
    if 'contributionsCollection' in member:
        contrib = member['contributionsCollection']
        member_dict[member['login']] = {
            'total_commits': contrib['totalCommitContributions'],
            'total_prs': contrib['totalPullRequestContributions'],
            'total_issues': contrib['totalIssueContributions'],
            'total_reviews': contrib['totalPullRequestReviewContributions']
        }

# Create team membership with contribution data
team_member_data = []
for team in team_data['teams']:
    if 'members' in team and 'nodes' in team['members']:
        for member in team['members']['nodes']:
            login = member['login']
            contrib = member_dict.get(login, {
                'total_commits': 0,
                'total_prs': 0,
                'total_issues': 0,
                'total_reviews': 0
            })

            team_member_data.append({
                'team_name': team['name'],
                'team_slug': team['slug'],
                'member_login': login,
                'member_name': member.get('name', login),
                'commits': contrib['total_commits'],
                'prs': contrib['total_prs'],
                'issues': contrib['total_issues'],
                'reviews': contrib['total_reviews']
            })

# Create a DataFrame for analysis
team_member_df = pd.DataFrame(team_member_data)

# Example: Calculate average contributions per team
team_summary = team_member_df.groupby('team_name').agg({
    'commits': 'mean',
    'prs': 'mean',
    'issues': 'mean',
    'reviews': 'mean',
    'member_login': 'count'
}).rename(columns={'member_login': 'member_count'})

print(team_summary)
```

## Use Cases

1. **Engineering Productivity Analysis**

   - Track team and individual contribution trends over time
   - Identify bottlenecks in the development process

2. **Team Composition Planning**

   - Analyze team sizes and distribution of responsibilities
   - Optimize team structures based on repository assignments

3. **Onboarding and Mentoring**

   - Identify experienced members who can mentor new team members
   - Track new member progress and integration

4. **Repository Ownership**

   - Clarify which teams own which repositories
   - Identify repositories that may need better team coverage

5. **Team Performance Metrics**

   - Compare contribution metrics across teams
   - Identify high-performing teams and successful team structures

6. **Individual Performance Tracking**
   - Monitor individual contribution patterns over time
   - Identify changes in activity levels

## Data Governance

When working with team and member data, follow these guidelines:

1. **Data Privacy**

   - Treat member email addresses and personal information as confidential
   - Follow company data governance policies

2. **Data Retention**

   - Implement appropriate retention policies for historical data
   - Consider anonymizing older datasets for long-term trend analysis

3. **Access Control**
   - Limit access to raw JSON data files
   - Consider creating sanitized views for general use

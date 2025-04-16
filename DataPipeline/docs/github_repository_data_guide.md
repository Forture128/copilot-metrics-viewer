# GitHub Repository Data Guide

This document explains the structure and content of the GitHub repository data collected by the GitHub Collector CLI.

## Overview

The GitHub Collector CLI's `collect` command gathers comprehensive repository data including:

- Basic repository metadata
- Pull requests
- Issues
- Commits
- Deployments
- Releases
- Security vulnerabilities
- DORA metrics
- Workflow runs

## Repository Data Structure

Repository data is stored in a JSON file with the following structure:

```json
{
  "organization": "moneyforward",
  "collected_at": "2025-04-06T12:34:56.789012",
  "repositories": {
    "repo-name": {
      "name": "repo-name",
      "url": "https://github.com/moneyforward/repo-name",
      "description": "Repository description",
      "createdAt": "2023-01-01T00:00:00Z",
      "updatedAt": "2024-01-01T00:00:00Z",
      "pullRequests": {
        "totalCount": 120,
        "nodes": [
          {
            "number": 42,
            "title": "Feature: Add new functionality",
            "createdAt": "2024-03-15T10:00:00Z",
            "mergedAt": "2024-03-16T15:30:00Z",
            "closedAt": "2024-03-16T15:30:00Z",
            "state": "MERGED",
            "additions": 150,
            "deletions": 20,
            "changedFiles": 5,
            "mergeCommit": {
              "oid": "abcdef1234567890",
              "committedDate": "2024-03-16T15:30:00Z"
            },
            "commits": {
              "totalCount": 3,
              "nodes": [
                {
                  "commit": {
                    "oid": "1234567890abcdef",
                    "committedDate": "2024-03-15T12:00:00Z",
                    "message": "Implement feature X"
                  }
                }
              ]
            },
            "reviews": {
              "totalCount": 2,
              "nodes": [
                {
                  "state": "APPROVED",
                  "submittedAt": "2024-03-16T14:00:00Z",
                  "author": {
                    "login": "reviewer1"
                  }
                }
              ]
            },
            "timelineItems": {
              "nodes": [
                {
                  "__typename": "MergedEvent",
                  "createdAt": "2024-03-16T15:30:00Z"
                }
              ]
            }
          }
        ]
      },
      "issues": {
        "totalCount": 50,
        "nodes": [
          {
            "number": 15,
            "title": "Bug: System crashes when X",
            "createdAt": "2024-02-20T09:00:00Z",
            "closedAt": "2024-02-22T16:45:00Z",
            "state": "CLOSED",
            "labels": {
              "nodes": [
                {
                  "name": "bug",
                  "color": "d73a4a"
                }
              ]
            },
            "timelineItems": {
              "nodes": [
                {
                  "__typename": "ClosedEvent",
                  "createdAt": "2024-02-22T16:45:00Z"
                }
              ]
            }
          }
        ]
      },
      "deployments": {
        "totalCount": 25,
        "nodes": [
          {
            "id": "DE_abc123",
            "state": "ACTIVE",
            "createdAt": "2024-03-17T10:00:00Z",
            "environment": "production",
            "latestStatus": {
              "state": "SUCCESS",
              "description": "Deployment completed",
              "createdAt": "2024-03-17T10:05:00Z",
              "logUrl": "https://github.com/moneyforward/repo-name/actions/runs/123456",
              "environmentUrl": "https://example.com"
            },
            "commit": {
              "oid": "abcdef1234567890",
              "message": "Merge pull request #42"
            },
            "ref": {
              "name": "main"
            },
            "task": "deploy",
            "payload": null
          }
        ]
      },
      "releases": {
        "totalCount": 10,
        "nodes": [
          {
            "name": "v1.0.0",
            "tagName": "v1.0.0",
            "createdAt": "2024-01-15T12:00:00Z",
            "publishedAt": "2024-01-15T14:00:00Z",
            "isPrerelease": false,
            "isDraft": false,
            "description": "Initial stable release"
          }
        ]
      },
      "vulnerabilityAlerts": {
        "nodes": [
          {
            "securityVulnerability": {
              "package": {
                "name": "vulnerable-package"
              },
              "severity": "HIGH",
              "vulnerableVersionRange": "<1.2.3",
              "firstPatchedVersion": {
                "identifier": "1.2.3"
              }
            },
            "createdAt": "2024-02-10T08:00:00Z",
            "dismissedAt": null,
            "fixedAt": null
          }
        ]
      },
      "defaultBranchRef": {
        "name": "main",
        "target": {
          "history": {
            "totalCount": 500,
            "nodes": [
              {
                "committedDate": "2024-03-16T15:30:00Z",
                "message": "Merge pull request #42",
                "changedFiles": 5,
                "additions": 150,
                "deletions": 20
              }
            ]
          },
          "checkSuites": {
            "nodes": [
              {
                "status": "COMPLETED",
                "conclusion": "SUCCESS",
                "workflowRun": {
                  "workflow": {
                    "name": "CI",
                    "id": "12345"
                  },
                  "runNumber": 42,
                  "updatedAt": "2024-03-16T15:40:00Z",
                  "url": "https://github.com/moneyforward/repo-name/actions/runs/123456",
                  "databaseId": 123456
                }
              }
            ]
          }
        }
      },
      "object": {
        "entries": [
          {
            "name": "workflows",
            "object": {
              "entries": [
                {
                  "name": "ci.yml",
                  "object": {
                    "text": "name: CI\n\non:\n  push:\n    branches: [ main ]\n  pull_request:\n    branches: [ main ]\n\njobs:\n  build:\n    runs-on: ubuntu-latest\n    steps:\n    - uses: actions/checkout@v2\n    - name: Run tests\n      run: |\n        npm ci\n        npm test\n"
                  }
                }
              ]
            }
          }
        ]
      }
    }
  }
}
```

## Key Repository Data Fields

### Basic Repository Information

- `name`: Repository name
- `url`: GitHub URL for the repository
- `description`: Description of the repository
- `createdAt`: Repository creation date
- `updatedAt`: Last update date

### Pull Requests

- `number`: PR number
- `title`: PR title
- `state`: Current state (OPEN, CLOSED, MERGED)
- `createdAt`: When the PR was created
- `mergedAt`: When the PR was merged (if applicable)
- `closedAt`: When the PR was closed
- `additions`/`deletions`/`changedFiles`: Change statistics
- `commits`: Commit history for the PR
- `reviews`: Code review information
- `timelineItems`: Timeline of PR events

### Issues

- `number`: Issue number
- `title`: Issue title
- `state`: Current state (OPEN, CLOSED)
- `createdAt`: When the issue was created
- `closedAt`: When the issue was closed (if applicable)
- `labels`: Issue labels
- `timelineItems`: Timeline of issue events

### Deployments

- `state`: Deployment state (ACTIVE, INACTIVE)
- `environment`: Deployment environment (e.g., production, staging)
- `latestStatus`: Current status of the deployment
- `commit`: The commit that was deployed
- `ref`: The Git reference (branch/tag) that was deployed

### Releases

- `name`: Release name
- `tagName`: Git tag for the release
- `createdAt`: When the release was created
- `publishedAt`: When the release was published
- `isPrerelease`: Whether this is a pre-release
- `isDraft`: Whether this is a draft release

### Security Vulnerabilities

- `securityVulnerability`: Details about the security vulnerability
- `createdAt`: When the alert was created
- `dismissedAt`: When the alert was dismissed (if applicable)
- `fixedAt`: When the vulnerability was fixed (if applicable)

### Default Branch Information

- `name`: Default branch name
- `target`: Information about the branch HEAD
  - `history`: Commit history
  - `checkSuites`: CI/CD runs

### Repository Files

- `object`: Repository content object
  - `entries`: File/directory entries

## DORA Metrics Calculation

DORA (DevOps Research and Assessment) metrics are calculated from the repository data:

1. **Deployment Frequency**
   - Calculated from the frequency of deployments to production
2. **Lead Time for Changes**
   - Time from commit to deployment in production
3. **Mean Time to Recovery (MTTR)**
   - Time between production incidents and their resolutions
4. **Change Failure Rate**
   - Percentage of deployments causing failures

## Data Analysis Examples

### Pull Request Analysis

Track PR activity, review patterns, and cycle times:

```python
import json
import pandas as pd
import matplotlib.pyplot as plt
from datetime import datetime

# Load repository data
with open('github_data_moneyforward_20250406_123456.json') as f:
    repo_data = json.load(f)

# Extract PR data from all repositories
all_prs = []
for repo_name, repo in repo_data['repositories'].items():
    if 'pullRequests' in repo and 'nodes' in repo['pullRequests']:
        for pr in repo['pullRequests']['nodes']:
            # Add repository name to the PR data
            pr_data = {
                'repo': repo_name,
                'number': pr['number'],
                'title': pr['title'],
                'state': pr['state'],
                'created_at': pr['createdAt'],
                'merged_at': pr['mergedAt'],
                'closed_at': pr['closedAt'],
                'additions': pr['additions'],
                'deletions': pr['deletions'],
                'changed_files': pr['changedFiles'],
                'review_count': pr['reviews']['totalCount'] if 'reviews' in pr else 0
            }

            # Calculate time to merge if merged
            if pr['state'] == 'MERGED' and pr['mergedAt'] and pr['createdAt']:
                created = datetime.fromisoformat(pr['createdAt'].replace('Z', '+00:00'))
                merged = datetime.fromisoformat(pr['mergedAt'].replace('Z', '+00:00'))
                pr_data['time_to_merge_hours'] = (merged - created).total_seconds() / 3600
            else:
                pr_data['time_to_merge_hours'] = None

            all_prs.append(pr_data)

# Create a DataFrame and analyze
pr_df = pd.DataFrame(all_prs)

# 1. PR Cycle Time Analysis
cycle_times = pr_df.dropna(subset=['time_to_merge_hours'])
avg_cycle_time = cycle_times.groupby('repo')['time_to_merge_hours'].mean()

plt.figure(figsize=(12, 6))
avg_cycle_time.plot(kind='bar', title='Average PR Cycle Time by Repository')
plt.ylabel('Hours')
plt.tight_layout()
plt.savefig('pr_cycle_time.png')

# 2. PR Size Analysis
pr_df['size_category'] = pd.cut(
    pr_df['additions'] + pr_df['deletions'],
    bins=[0, 50, 200, 500, float('inf')],
    labels=['Small', 'Medium', 'Large', 'X-Large']
)

size_vs_time = pr_df.groupby('size_category')['time_to_merge_hours'].mean().dropna()
size_vs_time.plot(kind='bar', title='PR Size vs. Time to Merge')
plt.ylabel('Hours')
plt.tight_layout()
plt.savefig('pr_size_vs_time.png')
```

### Deployment Analysis

Track deployment frequency and success rates:

```python
import json
import pandas as pd
import matplotlib.pyplot as plt
from datetime import datetime

# Load repository data
with open('github_data_moneyforward_20250406_123456.json') as f:
    repo_data = json.load(f)

# Extract deployment data from all repositories
all_deployments = []
for repo_name, repo in repo_data['repositories'].items():
    if 'deployments' in repo and 'nodes' in repo['deployments']:
        for deployment in repo['deployments']['nodes']:
            if 'environment' in deployment and deployment['environment'] == 'production':
                deploy_data = {
                    'repo': repo_name,
                    'id': deployment['id'],
                    'created_at': deployment['createdAt'],
                    'environment': deployment['environment'],
                    'state': deployment['state'],
                    'status': deployment['latestStatus']['state'] if 'latestStatus' in deployment else None,
                }
                all_deployments.append(deploy_data)

# Create a DataFrame and convert dates
deploy_df = pd.DataFrame(all_deployments)
if not deploy_df.empty:
    deploy_df['created_at'] = pd.to_datetime(deploy_df['created_at'])
    deploy_df['date'] = deploy_df['created_at'].dt.date

    # Count deployments by day and repository
    deploy_counts = deploy_df.groupby(['date', 'repo']).size().unstack().fillna(0)

    # Plot deployment frequency
    plt.figure(figsize=(15, 7))
    deploy_counts.plot(kind='bar', stacked=True, title='Production Deployments by Day')
    plt.ylabel('Number of Deployments')
    plt.tight_layout()
    plt.savefig('deployment_frequency.png')

    # Calculate deployment success rate
    deploy_df['success'] = deploy_df['status'] == 'SUCCESS'
    success_rate = deploy_df.groupby('repo')['success'].mean() * 100

    plt.figure(figsize=(12, 6))
    success_rate.plot(kind='bar', title='Deployment Success Rate by Repository')
    plt.ylabel('Success Rate (%)')
    plt.tight_layout()
    plt.savefig('deployment_success_rate.png')
```

### Code Velocity Analysis

Track team code velocity and commit patterns:

```python
import json
import pandas as pd
import matplotlib.pyplot as plt
from datetime import datetime

# Load repository data
with open('github_data_moneyforward_20250406_123456.json') as f:
    repo_data = json.load(f)

# Extract commit data from default branch histories
all_commits = []
for repo_name, repo in repo_data['repositories'].items():
    if ('defaultBranchRef' in repo and
        repo['defaultBranchRef'] and
        'target' in repo['defaultBranchRef'] and
        'history' in repo['defaultBranchRef']['target'] and
        'nodes' in repo['defaultBranchRef']['target']['history']):

        commits = repo['defaultBranchRef']['target']['history']['nodes']
        for commit in commits:
            commit_data = {
                'repo': repo_name,
                'date': datetime.fromisoformat(commit['committedDate'].replace('Z', '+00:00')),
                'message': commit['message'],
                'additions': commit['additions'],
                'deletions': commit['deletions'],
                'changed_files': commit['changedFiles'],
                'is_merge': commit['message'].startswith('Merge')
            }
            all_commits.append(commit_data)

# Create a DataFrame
commit_df = pd.DataFrame(all_commits)

if not commit_df.empty:
    # Add date components for grouping
    commit_df['day'] = commit_df['date'].dt.date
    commit_df['week'] = commit_df['date'].dt.isocalendar().week
    commit_df['month'] = commit_df['date'].dt.month
    commit_df['year'] = commit_df['date'].dt.year

    # Filter out merge commits for code velocity analysis
    non_merge_commits = commit_df[~commit_df['is_merge']]

    # Aggregate weekly code changes
    weekly_changes = non_merge_commits.groupby(['year', 'week']).agg({
        'additions': 'sum',
        'deletions': 'sum',
        'repo': 'count'  # Count of commits
    }).rename(columns={'repo': 'commit_count'})

    # Reset index for plotting
    weekly_changes = weekly_changes.reset_index()
    weekly_changes['week_label'] = weekly_changes['year'].astype(str) + '-W' + weekly_changes['week'].astype(str)

    # Plot weekly code velocity
    plt.figure(figsize=(15, 7))
    plt.bar(weekly_changes['week_label'], weekly_changes['additions'], label='Additions')
    plt.bar(weekly_changes['week_label'], -weekly_changes['deletions'], label='Deletions')
    plt.plot(weekly_changes['week_label'], weekly_changes['commit_count'] * 10, 'r-', label='Commit Count (x10)')

    plt.title('Weekly Code Velocity')
    plt.xlabel('Week')
    plt.ylabel('Lines of Code')
    plt.legend()
    plt.xticks(rotation=90)
    plt.tight_layout()
    plt.savefig('code_velocity.png')
```

### Issue Analysis

Track issue resolution times and patterns:

```python
import json
import pandas as pd
import matplotlib.pyplot as plt
from datetime import datetime

# Load repository data
with open('github_data_moneyforward_20250406_123456.json') as f:
    repo_data = json.load(f)

# Extract issue data from all repositories
all_issues = []
for repo_name, repo in repo_data['repositories'].items():
    if 'issues' in repo and 'nodes' in repo['issues']:
        for issue in repo['issues']['nodes']:
            # Extract label names
            labels = []
            if 'labels' in issue and 'nodes' in issue['labels']:
                labels = [label['name'] for label in issue['labels']['nodes']]

            issue_data = {
                'repo': repo_name,
                'number': issue['number'],
                'title': issue['title'],
                'state': issue['state'],
                'created_at': issue['createdAt'],
                'closed_at': issue['closedAt'],
                'labels': labels,
                'is_bug': 'bug' in labels
            }

            # Calculate time to close if closed
            if issue['state'] == 'CLOSED' and issue['closedAt'] and issue['createdAt']:
                created = datetime.fromisoformat(issue['createdAt'].replace('Z', '+00:00'))
                closed = datetime.fromisoformat(issue['closedAt'].replace('Z', '+00:00'))
                issue_data['time_to_close_hours'] = (closed - created).total_seconds() / 3600
            else:
                issue_data['time_to_close_hours'] = None

            all_issues.append(issue_data)

# Create a DataFrame
issue_df = pd.DataFrame(all_issues)

if not issue_df.empty:
    # Convert date strings to datetime
    issue_df['created_at'] = pd.to_datetime(issue_df['created_at'])

    # Issue resolution time analysis
    closed_issues = issue_df.dropna(subset=['time_to_close_hours'])

    # Compare bug vs. non-bug resolution times
    bug_resolution = closed_issues.groupby('is_bug')['time_to_close_hours'].mean()

    plt.figure(figsize=(10, 6))
    bug_resolution.plot(kind='bar', title='Average Resolution Time: Bugs vs. Non-Bugs')
    plt.ylabel('Hours')
    plt.xticks([0, 1], ['Non-Bug', 'Bug'], rotation=0)
    plt.tight_layout()
    plt.savefig('bug_resolution_time.png')

    # Track open vs. closed issues over time
    issue_df['month'] = issue_df['created_at'].dt.to_period('M')
    monthly_issues = issue_df.groupby(['month', 'state']).size().unstack().fillna(0)

    plt.figure(figsize=(15, 7))
    monthly_issues.plot(kind='bar', stacked=True, title='Issues by Month and State')
    plt.ylabel('Number of Issues')
    plt.tight_layout()
    plt.savefig('monthly_issues.png')
```

## Combining Repository, Team, and Member Data

To get a comprehensive view, you can combine repository data with team and member data:

```python
import json
import pandas as pd

# Load repository, team, and member data
with open('github_data_moneyforward_20250406_123456.json') as f:
    repo_data = json.load(f)

with open('github_teams_moneyforward_20250406_123456.json') as f:
    team_data = json.load(f)

with open('github_members_moneyforward_20250406_123456.json') as f:
    member_data = json.load(f)

# Create repository to team mapping
repo_team_map = {}
for team in team_data['teams']:
    if 'repositories' in team and 'nodes' in team['repositories']:
        for repo in team['repositories']['nodes']:
            repo_name = repo['name']
            if repo_name not in repo_team_map:
                repo_team_map[repo_name] = []
            repo_team_map[repo_name].append(team['name'])

# Create member contribution dict
member_contrib = {}
for member in member_data['members']:
    if 'contributionsCollection' in member:
        login = member['login']
        contrib = member['contributionsCollection']

        # Get repository-specific contributions
        repo_contributions = {}
        if 'commitContributionsByRepository' in contrib:
            for repo_contrib in contrib['commitContributionsByRepository']:
                repo_name = repo_contrib['repository']['name']
                count = repo_contrib['contributions']['totalCount']
                repo_contributions[repo_name] = count

        member_contrib[login] = {
            'name': member.get('name', login),
            'total_commits': contrib.get('totalCommitContributions', 0),
            'total_prs': contrib.get('totalPullRequestContributions', 0),
            'repos': repo_contributions
        }

# Extract pull request data with team and contributor info
enhanced_prs = []
for repo_name, repo in repo_data['repositories'].items():
    if 'pullRequests' in repo and 'nodes' in repo['pullRequests']:
        for pr in repo['pullRequests']['nodes']:
            if 'author' in pr and pr['author']:
                author_login = pr['author'].get('login')

                pr_data = {
                    'repo': repo_name,
                    'number': pr['number'],
                    'title': pr['title'],
                    'state': pr['state'],
                    'author': author_login,
                    'author_name': member_contrib.get(author_login, {}).get('name', author_login),
                    'teams': repo_team_map.get(repo_name, []),
                    'created_at': pr['createdAt'],
                    'merged_at': pr['mergedAt'],
                    'closed_at': pr['closedAt'],
                    'additions': pr['additions'],
                    'deletions': pr['deletions'],
                    'changed_files': pr['changedFiles']
                }
                enhanced_prs.append(pr_data)

# Create a DataFrame for team-based PR analysis
pr_df = pd.DataFrame(enhanced_prs)

if not pr_df.empty and 'teams' in pr_df.columns:
    # Explode the teams column to analyze PRs by team
    exploded_pr_df = pr_df.explode('teams')

    # Analyze PR velocity by team
    team_velocity = exploded_pr_df.groupby('teams').agg({
        'number': 'count',
        'additions': 'sum',
        'deletions': 'sum',
    }).rename(columns={'number': 'pr_count'})

    print("Pull Request Activity by Team:")
    print(team_velocity)

    # Analyze contributor activity within teams
    team_contributor = exploded_pr_df.groupby(['teams', 'author']).agg({
        'number': 'count'
    }).rename(columns={'number': 'pr_count'})

    print("\nTop Contributors by Team:")
    for team in team_contributor.index.get_level_values(0).unique():
        print(f"\n{team}:")
        print(team_contributor.loc[team].sort_values('pr_count', ascending=False).head(5))
```

## Use Cases

### Engineering Performance

1. **DORA Metrics Dashboard**

   - Visualize all four DORA metrics over time
   - Identify trends and areas for improvement

2. **Pull Request Efficiency**

   - Track PR cycle time trends
   - Identify bottlenecks in the review process
   - Find optimal PR sizes and review patterns

3. **Release Cadence**
   - Analyze deployment and release frequencies
   - Track stability of releases

### Quality Management

1. **Bug Analysis**

   - Track bug discovery and resolution rates
   - Identify components with high defect rates

2. **Security Posture**

   - Monitor vulnerability counts and resolution times
   - Track security patch adoption

3. **Test Coverage**
   - Analyze CI/CD results
   - Track test-related metrics

### Collaboration Insights

1. **Cross-Team Collaboration**

   - Identify repositories with multiple team contributions
   - Analyze shared ownership patterns

2. **Knowledge Concentration**
   - Identify repositories with high bus factor risk
   - Track knowledge distribution across the team

## Data Governance

Follow these guidelines when working with repository data:

1. **Data Privacy**

   - Be cautious with commit messages that might contain sensitive information
   - Respect privacy when analyzing individual contributor metrics

2. **Data Volume Management**

   - Repository data can be quite large due to PR and commit history
   - Consider implementing data retention policies for historical data

3. **Rate Limiting**
   - Be aware of GitHub API rate limits when collecting data
   - Use incremental collection for large repositories

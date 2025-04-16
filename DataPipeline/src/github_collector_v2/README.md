# GitHub Collector V2

A modern, asynchronous GitHub data collector with improved caching and concurrency support.

## Features

- **Asynchronous**: Uses Python's asyncio for efficient concurrency
- **GraphQL-based**: Leverages GitHub's GraphQL API for efficient data fetching
- **Caching**: Multi-level caching with memory and Redis support
- **Rate-limit aware**: Automatically respects GitHub's rate limits
- **Fault tolerant**: Automatic retries with exponential backoff
- **Modular**: Well-organized code for easy extension
- **Separation of concerns**: Data collection and metrics calculation are separate processes

## Installation

1. Install the required packages:

```bash
pip install -r requirements.txt
```

2. Set up your GitHub token:

```bash
export GITHUB_TOKEN=your_github_token
```

3. (Optional) For Redis caching, install Redis and set environment variables:

```bash
export USE_REDIS=true         # Enable Redis caching
export REDIS_HOST=localhost   # Redis host (default: localhost)
export REDIS_PORT=6379        # Redis port (default: 6379)
export REDIS_DB=0             # Redis database number (default: 0)
export REDIS_PASSWORD=secret  # Redis password (if required)
```

## Usage

The workflow is split into two main steps:

1. **Data Collection**: Fetch data from GitHub API
2. **Metrics Calculation**: Process collected data to generate DORA metrics

### Step 1: Data Collection

#### CLI Commands

##### Collect repository data for an organization

```bash
python -m src.github_collector_v2.cli collect <org_name> [OPTIONS]
```

Options:

- `--limit INTEGER`: Limit number of repositories to collect
- `--batch-size INTEGER`: Number of repositories to process in parallel
- `--output-dir PATH`: Output directory
- `--use-redis/--no-redis`: Enable/disable Redis caching
- `--redis-host TEXT`: Redis host
- `--redis-port INTEGER`: Redis port
- `--redis-db INTEGER`: Redis database number
- `--clear-cache`: Clear cache before collecting data

##### Collect team data for an organization

```bash
python -m src.github_collector_v2.cli collect-teams <org_name> [OPTIONS]
```

Options:

- `--output-dir PATH`: Output directory
- `--use-redis/--no-redis`: Enable/disable Redis caching
- `--redis-host TEXT`: Redis host
- `--redis-port INTEGER`: Redis port
- `--redis-db INTEGER`: Redis database number
- `--clear-cache`: Clear cache before collecting data
- `--include-members`: Include members in team data (default: True)

##### Collect member data for an organization

```bash
python -m src.github_collector_v2.cli collect-members <org_name> [OPTIONS]
```

Options:

- `--output-dir PATH`: Output directory
- `--use-redis/--no-redis`: Enable/disable Redis caching
- `--redis-host TEXT`: Redis host
- `--redis-port INTEGER`: Redis port
- `--redis-db INTEGER`: Redis database number
- `--clear-cache`: Clear cache before collecting data
- `--detailed-info`: Include detailed member information (default: True)
- `--batch-size INTEGER`: Number of members to process in parallel (default: 10)

##### Collect member contributions for specific members

```bash
python -m src.github_collector_v2.cli collect-members-contributions <org_name> [OPTIONS]
```

Options:

- `--output-dir PATH`: Output directory
- `--use-redis/--no-redis`: Enable/disable Redis caching
- `--redis-host TEXT`: Redis host
- `--redis-port INTEGER`: Redis port
- `--redis-db INTEGER`: Redis database number
- `--clear-cache`: Clear cache before collecting data
- `--team-file PATH`: Path to team JSON file to extract members from
- `--team-csv PATH`: Path to team members CSV file to extract logins from
- `--batch-size INTEGER`: Number of members to process in parallel (default: 10)

This command is useful for collecting contributions data for a specific subset of members, rather than all organization members. It can extract member logins from either a team JSON file or a CSV file.

##### Test data collection for a single repository

```bash
python -m src.github_collector_v2.cli test <org_name> <repo_name> [OPTIONS]
```

Options:

- `--use-redis/--no-redis`: Enable/disable Redis caching
- `--redis-host TEXT`: Redis host
- `--redis-port INTEGER`: Redis port
- `--redis-db INTEGER`: Redis database number
- `--output PATH`: Save repository data to JSON file
- `--force-refresh`: Force refresh from API (ignore cache)

##### Get information about an organization

```bash
python -m src.github_collector_v2.cli info <org_name> [OPTIONS]
```

Options:

- `--use-redis/--no-redis`: Enable/disable Redis caching
- `--redis-host TEXT`: Redis host
- `--redis-port INTEGER`: Redis port
- `--redis-db INTEGER`: Redis database number

##### Clear cache

```bash
python -m src.github_collector_v2.cli clear-cache <org_name> [OPTIONS]
```

Options:

- `--use-redis/--no-redis`: Enable/disable Redis caching
- `--redis-host TEXT`: Redis host
- `--redis-port INTEGER`: Redis port
- `--redis-db INTEGER`: Redis database number
- `--prefix TEXT`: Clear only keys with this prefix

### Step 2: DORA Metrics Calculation

#### CLI Commands

##### Calculate metrics for a single data file

```bash
python -m src.github_collector_v2.dora_cli calculate <input_file> [OPTIONS]
```

Options:

- `--output, -o PATH`: Path to save the metrics to
- `--days, -d INTEGER`: Time period in days to calculate metrics (default: 90)

##### Calculate metrics for multiple data files

```bash
python -m src.github_collector_v2.dora_cli batch_calculate <directory> [OPTIONS]
```

Options:

- `--pattern, -p TEXT`: File pattern to match (default: _github*data*_.json)
- `--output-dir PATH`: Directory to save metrics to
- `--days, -d INTEGER`: Time period in days to calculate metrics (default: 90)

### Programmatic Usage

#### Repository Data Collection

```python
import asyncio
from src.github_collector_v2.collector import GitHubCollector

async def main():
    # Create collector from environment variables
    collector = GitHubCollector.from_env(org_name="github")

    # Or manually specify all options
    collector = GitHubCollector(
        token="your_github_token",
        org_name="github",
        use_redis=True,
        redis_host="localhost",
        redis_port=6379,
        redis_db=0,
        cache_ttl=3600,
        rate_limit_buffer=100,
        rate_limit_check_interval=60  # Check rate limit every 60 seconds
    )

    async with collector:
        # Get organization information
        org_data = await collector.get_organization()
        print(f"Organization: {org_data.get('name')}")

        # Get repositories
        repos = await collector.get_repositories(limit=5)
        print(f"Found {len(repos)} repositories")

        # Collect detailed data
        data = await collector.collect_dora_metrics(
            repos=[repo["name"] for repo in repos],
            output_file="data/github_data.json",
            batch_size=2
        )

if __name__ == "__main__":
    asyncio.run(main())
```

#### Team and Member Data Collection

```python
import asyncio
from src.github_collector_v2.collector import GitHubCollector
import json

async def collect_team_and_member_data():
    # Create collector from environment variables
    collector = GitHubCollector.from_env(org_name="github")

    async with collector:
        # Collect team data
        teams = await collector.get_teams(include_members=True)
        print(f"Found {len(teams)} teams")

        # Save team data
        with open("data/github_teams.json", "w") as f:
            json.dump({
                "organization": "github",
                "collected_at": datetime.utcnow().isoformat(),
                "teams": teams,
                "metadata": {
                    "team_count": len(teams),
                    "include_members": True
                }
            }, f, indent=2)

        # Collect member data
        members = await collector.get_organization_members(include_detailed_info=True)
        print(f"Found {len(members)} members")

        # Save member data
        with open("data/github_members.json", "w") as f:
            json.dump({
                "organization": "github",
                "collected_at": datetime.utcnow().isoformat(),
                "members": members,
                "metadata": {
                    "member_count": len(members),
                    "include_detailed_info": True
                }
            }, f, indent=2)

        # Collect contributions for specific members
        member_logins = ["user1", "user2", "user3"]
        member_contributions = await collector.collect_user_contributions(
            member_logins,
            batch_size=10
        )
        print(f"Collected contributions for {len(member_contributions)} members")

        # Save member contributions
        with open("data/github_member_contributions.json", "w") as f:
            json.dump({
                "organization": "github",
                "collected_at": datetime.utcnow().isoformat(),
                "members_contributions": member_contributions,
                "metadata": {
                    "member_count": len(member_contributions)
                }
            }, f, indent=2)

if __name__ == "__main__":
    asyncio.run(collect_team_and_member_data())
```

#### Metrics Calculation

```python
from src.github_collector_v2.dora_metrics import (
    load_github_data,
    calculate_dora_metrics,
    get_four_key_metrics,
    classify_performance_level
)

def main():
    # Load data collected by the GitHub collector
    data = load_github_data("data/github_data.json")

    # Calculate DORA metrics
    metrics = calculate_dora_metrics(
        data,
        time_period_days=90,
        output_file="data/dora_metrics.json"
    )

    # Get the four key metrics
    four_keys = get_four_key_metrics(metrics)
    print("Deployment Frequency:", four_keys["deployment_frequency_per_week"], "per week")
    print("Lead Time for Changes:", four_keys["lead_time_for_changes_hours"], "hours")
    print("Change Failure Rate:", four_keys["change_failure_rate"] * 100, "%")
    print("Mean Time to Recover:", four_keys["mean_time_to_recover_hours"], "hours")

    # Classify performance level
    performance = classify_performance_level(four_keys)
    print("Performance Levels:", performance)

if __name__ == "__main__":
    main()
```

## Cache Architecture

The collector implements a two-level caching architecture:

1. **Memory Cache**: Fast in-memory cache with TTL (Time-To-Live)
2. **Redis Cache**: Distributed cache for sharing between processes and persistence

When Redis is enabled, the cache lookup works as follows:

1. Check Redis first
2. If found in Redis, return result and also update memory cache
3. If not found in Redis, check memory cache
4. If not found anywhere, fetch from GitHub API
5. Store result in both Redis and memory cache

## Rate Limit Management

The collector manages GitHub API rate limits intelligently:

1. **Periodic Checking**: Checks rate limits every N batches or after a configurable time interval
2. **Adaptive Frequency**: Increases check frequency when close to the limit
3. **Smart Waiting**: Implements a progressive wait strategy for long waits with periodic updates
4. **Resource Efficient**: Avoids unnecessary API calls to check rate limits

Configuration options:

- `rate_limit_buffer`: Number of remaining calls to keep in reserve (default: 100)
- `rate_limit_check_interval`: Minimum seconds between rate limit checks (default: 60)

## Data Collection

The collector fetches the following data from GitHub:

1. **Organization information**
2. **Repository list and metadata**
3. **Repository details**:
   - Pull requests
   - Issues
   - Deployments
   - Releases
   - Commit history
4. **Team structure**:
   - Team hierarchy
   - Team members
   - Team repository assignments
5. **Member data**:
   - Basic profile information
   - Contribution statistics
   - Repository-specific contributions

## DORA Metrics

The metrics calculator calculates the following DORA (DevOps Research and Assessment) metrics:

1. **Deployment Frequency**: How often code is deployed to production
2. **Lead Time for Changes**: Time from code commit to deployment
3. **Change Failure Rate**: Percentage of deployments that cause a failure
4. **Mean Time to Recovery**: Time to recover from a failure

## License

MIT

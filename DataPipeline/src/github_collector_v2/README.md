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

##### Collect data for an organization

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

#### Data Collection

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
        rate_limit_buffer=100
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

## DORA Metrics

The metrics calculator calculates the following DORA (DevOps Research and Assessment) metrics:

1. **Deployment Frequency**: How often code is deployed to production
2. **Lead Time for Changes**: Time from code commit to deployment
3. **Change Failure Rate**: Percentage of deployments that cause a failure
4. **Time to Restore Service**: Time to recover from an incident

## License

MIT

# GitHub Data Collector

A comprehensive data collection tool for GitHub using PyGithub and GraphQL APIs, with features like concurrency, rate limiting, fault tolerance, and caching.

## Features

- Collect data from GitHub using both REST and GraphQL APIs
- Rate limiting awareness and automatic handling
- Concurrent data collection for better performance
- Fault tolerance with exponential backoff and retry mechanisms
- Caching of API responses to reduce API calls
- Multiple collection modes (basic, extended, GraphQL)
- Comprehensive CLI interface

## Components

### Component Overview

- `collector.py`: Base collector with rate limiting, caching, and fault tolerance.
- `extended_collector.py`: Extended collector for additional GitHub objects like workflows, teams, releases, etc.
- `graphql_collector.py`: GraphQL-based collector for more efficient data collection.
- `cli.py`: A unified command-line interface for all collectors.
- `__init__.py`: Proper package initialization with exports.
- `README.md`: Documentation on how to use the GitHub collector.

### Key Features Summary

- **Comprehensive API Coverage**: Covers a wide range of GitHub API operations through both REST and GraphQL APIs.
- **Concurrency**: Utilizes ThreadPoolExecutor and asyncio for concurrent data collection, enhancing performance.
- **Rate Limiting**: Collectors are aware of GitHub's rate limits and automatically wait when limits are nearly reached to avoid errors.
- **Fault Tolerance**: Implements error handling with automatic retries using exponential backoff for transient failures.
- **Caching**: Caches all API responses to reduce the number of API calls made to GitHub.
- **Flexible Interface**: CLI offers various collection modes (basic, extended, GraphQL) and targets (organizations, repositories).
- **Best Practices**: Follows GitHub's REST API documentation best practices, including proper rate limit handling and efficient data retrieval patterns using concurrent requests. Data is stored in JSON format, easily loadable into data lakes or databases for further analysis.

### Note: Use Case of CLI | Develop & Verify Purpose

The `cli.py` in your GitHub collector package serves a different purpose from Airflow's DAGs. Here's when and how to use each:

**1. GitHub Collector CLI (`cli.py`)**:

- Use this when you want to collect GitHub data directly from the command line, outside of Airflow
- Useful for:
  - Testing the collector functionality
  - One-off data collection tasks
  - Local development and debugging
  - Quick data pulls without setting up Airflow

Example usage of GitHub collector CLI:

```bash
# For extended collection:
python -m src.github_collector.cli extended --org moneyforward --log-level DEBUG

# For basic collection:
python -m src.github_collector.cli base --org moneyforward

# For GraphQL collection:
python -m src.github_collector.cli graphql --org moneyforward

# For all methods:
python -m src.github_collector.cli all --org moneyforward
```

**2. Airflow DAG (`github_collector_dag.py`)**:

- Use this when you need:
  - Scheduled data collection
  - Automated workflow
  - Monitoring and alerting
  - Integration with other data pipelines
  - Retries and error handling at the workflow level

To check if your DAG is properly loaded in Airflow, you can use these Airflow CLI commands:

```bash
# List all DAGs
docker-compose -f docker/docker-compose.yml exec airflow airflow dags list

# Test the DAG file
docker-compose -f docker/docker-compose.yml exec airflow airflow dags test github_data_collection

# Check for import errors
docker-compose -f docker/docker-compose.yml exec airflow airflow dags list-import-errors
```

**3. When to Use Which**:

Use the GitHub Collector CLI when:

- You're developing/testing the collector
- You need a quick one-time data pull
- You want to test different collection configurations
- You're debugging collection issues

Use the Airflow DAG when:

- You need regular scheduled collection
- You want monitoring and alerting
- The collection is part of a larger data pipeline
- You need workflow-level retries and error handling

[Source: [Airflow CLI Documentation](https://airflow.apache.org/docs/apache-airflow/stable/howto/usage-cli.html)]

Currently, I notice your DAG has an import error. To fix this and get your DAG showing up in Airflow, you should:

1. Check if the plugin is properly installed:

```bash
docker-compose -f docker/docker-compose.yml exec airflow ls -l /opt/airflow/plugins/github_collector_utils.py
```

2. Verify the DAG syntax:

```bash
docker-compose -f docker/docker-compose.yml exec airflow python3 /opt/airflow/dags/github_collector_dag.py
```

This way you can use both tools effectively - the CLI for development/testing and the DAG for production automation.

## Installation

Ensure you have PyGithub installed:

```bash
pip install PyGithub
```

## GitHub Token

You need a GitHub Personal Access Token with appropriate scopes to use this collector. The token can be provided in three ways:

1. As a command-line argument: `--token YOUR_TOKEN`
2. As an environment variable: `export GITHUB_TOKEN=YOUR_TOKEN`
3. In your `.env` file: `GITHUB_TOKEN=YOUR_TOKEN`

## Usage

### Basic Collection

Collect basic repository data for an organization:

```bash
python -m src.github_collector.cli base --org your-organization
```

### Extended Collection

Collect additional data including workflows, releases, branches, and more:

```bash
python -m src.github_collector.cli extended --org your-organization
```

### GraphQL Collection

Use GraphQL API for efficient collection:

```bash
python -m src.github_collector.cli graphql --org your-organization
```

### All Collection Methods

Run all collection methods and combine the results:

```bash
python -m src.github_collector.cli all --org your-organization
```

### Additional Options

```
--token TOKEN           GitHub personal access token
--org ORG               GitHub organization name
--repo REPO             GitHub repository in owner/repo format
--output-dir DIR        Directory to save output files (default: data/github)
--no-cache              Disable caching of API responses
--log-level LEVEL       Set logging level (DEBUG, INFO, WARNING, ERROR, CRITICAL)
--concurrent NUM        Number of concurrent API requests
```

## Examples

### Collect Organization Data

```bash
python -m src.github_collector.cli extended --org octocat --output-dir ./data/octocat
```

### Collect Repository Data

```bash
python -m src.github_collector.cli graphql --repo octocat/Hello-World
```

### Disable Caching

```bash
python -m src.github_collector.cli base --org octocat --no-cache
```

## Programmatic Usage

You can also use the collectors programmatically:

```python
from src.github_collector.collector import GitHubCollector
from src.github_collector.extended_collector import ExtendedGitHubCollector
from src.github_collector.graphql_collector import GitHubGraphQLCollector

# Basic collector
collector = GitHubCollector(token="your-token")
data = collector.collect_all_data_sync("your-organization")
collector.save_data_to_json(data, "output.json")

# Extended collector
ext_collector = ExtendedGitHubCollector(token="your-token")
ext_data = ext_collector.collect_extended_data_sync("your-organization")

# GraphQL collector
gql_collector = GitHubGraphQLCollector(token="your-token")
gql_data = gql_collector.collect_organization_data_sync("your-organization")
```

Set up Airflow variables:
Apply to README.md
Run

```bash

airflow variables set github_org "your-org-name"
airflow variables set github_data_dir "data/github"
airflow variables set github_concurrent_requests "5"
airflow variables set github_use_cache "true"
airflow variables set github_retention_days "30"

```

## Rate Limiting

The collector respects GitHub's rate limits and will automatically wait if limits are nearly reached. You can adjust the buffer using the `rate_limit_buffer` parameter when initializing collectors.

## Fault Tolerance

The collectors handle errors and include automatic retry logic with exponential backoff for transient failures.

## Data Storage

By default, collected data is saved to JSON files in the `data/github` directory, but you can customize this with the `--output-dir` option.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

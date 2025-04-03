# Airflow DAGs

This directory contains Airflow DAGs and plugins for automated data collection and processing.

## Structure

```
airflow/
├── dags/                # DAG definitions
│   └── github_collector_v2_dag.py  # GitHub data collection DAG
├── plugins/             # Custom Airflow plugins
│   └── utils/          # Utility functions and helpers
├── config/             # Airflow configuration
└── logs/               # Airflow logs
```

## Environment Setup

### Development Environment

For local development, use environment variables:

```bash
export GITHUB_TOKEN=your-token
export USE_REDIS=true
export REDIS_HOST=localhost
export REDIS_PORT=6379
```

### Docker Environment

In Docker, we use Airflow Variables instead of environment variables:

1. Set variables through Airflow UI:
   - `github_org`
   - `github_repo_limit`
   - `github_batch_size`
2. Secrets like `GITHUB_TOKEN` should be set through Airflow Connections or Secrets Backend

### Important Note

- Development environment uses `.env` for configuration
- Docker environment uses Airflow Variables and Connections
- DO NOT include `.env` files in Docker images
- Keep Docker images minimal with only required packages

## DAGs

### GitHub Collector V2 DAG

The `github_collector_v2_dag.py` DAG collects GitHub organization data including:

- Repository data with detailed PR and commit information
- Team structure and membership data
- User contribution metrics

Features:

- Parallel data collection tasks
- Redis caching support
- Date-based file organization
- Automatic symlink management

Configuration via Airflow Variables:

- `github_org`: GitHub organization name
- `github_repo_limit`: Maximum repositories to collect (0 for all)
- `github_batch_size`: Number of repositories to process in parallel

## Setup

### Local Development Setup

```bash
# Install all dependencies including development tools
pip install -r dev-requirements.txt

# Set up environment variables
cp .env.example .env
# Edit .env with your values
```

### Docker Setup

```bash
# Build with minimal requirements
docker build -f Dockerfile.airflow -t github-collector-airflow .

# Run Airflow
docker-compose up -d

# Set up Variables in Airflow UI
- Navigate to Admin -> Variables
- Add required variables
```

## Usage

### Local Development

1. Start Airflow in local mode:

```bash
export AIRFLOW_HOME=$(pwd)/airflow
airflow standalone
```

2. Access UI at http://localhost:8080

### Docker Environment

1. Start services:

```bash
docker-compose up -d
```

2. Configure through Airflow UI:
   - Set Variables
   - Configure Connections
   - Enable DAGs

## Development Guidelines

### Adding New DAGs

1. Create DAG file in `dags/`
2. Use Airflow Variables for configuration
3. Avoid environment variables in DAG code
4. Keep dependencies minimal
5. Document required Variables and Connections

### Testing

1. Local testing with environment variables
2. Docker testing with Airflow Variables
3. Use different configurations for dev/prod

### Monitoring

- View DAG runs in Airflow UI
- Check logs in `logs/` directory
- Monitor task status and durations
- Configure alerts for failures

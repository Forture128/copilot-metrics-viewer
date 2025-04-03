# CLI Commands

This module contains all command-line interface (CLI) tools for the DataPipeline project.

## Structure

```
cli/
├── github/              # GitHub-related CLI commands
│   ├── collector_cli.py # GitHub data collection commands
│   └── dora_cli.py     # DORA metrics commands
└── README.md           # This file
```

## GitHub CLI Commands

### Collector CLI

The collector CLI provides commands for collecting GitHub data:

```bash
# Collect repository data
python -m cli.github.collector_cli collect-repos --org your-org-name

# Collect with Redis caching
python -m cli.github.collector_cli collect-repos --org your-org-name --use-redis
```

### DORA CLI

The DORA CLI provides commands for calculating and analyzing DORA metrics:

```bash
# Calculate DORA metrics
python -m cli.github.dora_cli calculate --input data/github/latest/github_data_latest.json

# Generate DORA report
python -m cli.github.dora_cli report --input data/github/latest/github_data_latest.json
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

### Configuration Files

Configuration can also be provided via YAML files:

```yaml
github:
  token: your-token
  org: your-org
redis:
  enabled: true
  host: localhost
  port: 6379
  db: 0
```

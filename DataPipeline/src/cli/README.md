# CLI Commands

This module contains all command-line interface (CLI) tools for the DataPipeline project.

## Structure

```
cli/
├── github/              # GitHub-related CLI commands
│   ├── collector_cli.py # GitHub data collection commands
│   ├── team_analysis.py # GitHub team and member analysis commands
│   └── README.md        # GitHub CLI documentation
└── README.md           # This file
```

## GitHub CLI Commands

### Collector CLI

The collector CLI provides commands for collecting GitHub data:

```bash
# Collect repository data
python -m src.cli.github.collector_cli collect-repository --org your-org-name

# Collect with Redis caching
python -m src.cli.github.collector_cli collect-repository --org your-org-name --use-redis

# Collect team data
python -m src.cli.github.collector_cli collect-teams --org your-org-name

# Collect member data
python -m src.cli.github.collector_cli collect-members --org your-org-name

# Test data collection for a single repository
python -m src.cli.github.collector_cli test --org your-org-name --repo your-repo-name

# Show cache configuration
python -m src.cli.github.collector_cli info --org your-org-name --use-redis

# Clear cache
python -m src.cli.github.collector_cli clear-cache --org your-org-name --use-redis

```

### Team Analysis CLI

The team analysis CLI provides commands for analyzing GitHub team and member data:

```bash
# Analyze team data
python -m src.cli.github.team_analysis analyze-teams --org your-org-name --team-file data/github/github_teams_your-org-name_20230101_120000.json

# Perform comprehensive analysis of team and member data
python -m src.cli.github.team_analysis comprehensive-analysis --org your-org-name --team-file data/github/github_teams_your-org-name_20230101_120000.json --member-file data/github/github_members_your-org-name_20230101_120000.json

# Extract member contributions to CSV
python -m src.cli.github.team_analysis extract-members --member-file data/github/github_members_your-org-name_20230101_120000.json

# Extract team members to CSV with options
python -m src.cli.github.team_analysis extract-team-members --team-file data/github/github_teams_your-org-name_20230101_120000.json --list-unique --sort-by-teams
```

More details are available in the [GitHub CLI README](./github/README.md).

<!-- ### DORA CLI

The DORA CLI provides commands for calculating and analyzing DORA metrics:

```bash
# Calculate DORA metrics
python -m src.cli.github.dora_cli calculate --input data/github/latest/github_data_latest.json

# Generate DORA report
python -m src.cli.github.dora_cli report --input data/github/latest/github_data_latest.json
``` -->

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
  token: your-github-token
  cache:
    use_redis: true
    redis_host: localhost
    redis_port: 6379
    redis_db: 0
```

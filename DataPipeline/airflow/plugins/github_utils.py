"""
GitHub utilities for Airflow DAGs
"""

from airflow.plugins_manager import AirflowPlugin
from pathlib import Path
import json
import os


def get_data_dir():
    """Get the data directory from environment or use default"""
    return Path(os.environ.get("DATA_DIR", "/opt/airflow/data"))


def save_github_data(data, file_path):
    """Save GitHub data to file"""
    # Ensure directory exists
    file_path.parent.mkdir(parents=True, exist_ok=True)

    # Write data to file
    with open(file_path, "w") as f:
        json.dump(data, f, indent=2)

    return file_path


def load_github_data(file_path, default=None):
    """Load GitHub data from file"""
    if not file_path.exists():
        return default if default is not None else {}

    with open(file_path, "r") as f:
        return json.load(f)


class GitHubUtilsPlugin(AirflowPlugin):
    name = "github_utils"
    macros = [get_data_dir, save_github_data, load_github_data]

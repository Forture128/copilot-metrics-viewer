"""
Path utilities for Airflow DAGs
"""

from datetime import datetime
from pathlib import Path
from typing import Tuple


def get_date_based_paths(
    base_dir: Path,
    date: datetime,
    latest_dir_name: str = "latest",
    create_dirs: bool = True,
) -> Tuple[Path, Path]:
    """
    Generate date-based directory structure and latest symlink directory

    Args:
        base_dir: Base directory for data storage
        date: Date to create directory structure for
        latest_dir_name: Name of the directory for latest symlinks (default: "latest")
        create_dirs: Whether to create the directories if they don't exist (default: True)

    Returns:
        Tuple of (date_path, latest_dir)
        - date_path: Path object for the date-based directory
        - latest_dir: Path object for the latest symlinks directory
    """
    # Create date-based path
    date_path = base_dir / str(date.year) / f"{date.month:02d}" / f"{date.day:02d}"

    # Create latest symlink directory path
    latest_dir = base_dir / latest_dir_name

    if create_dirs:
        # Create directories if they don't exist
        date_path.mkdir(parents=True, exist_ok=True)
        latest_dir.mkdir(exist_ok=True)

    return date_path, latest_dir


def update_symlink(current_file: Path, latest_file: Path) -> None:
    """
    Update a symlink to point to the current file

    Args:
        current_file: Path to the current file
        latest_file: Path to the symlink file to update
    """
    if latest_file.exists():
        latest_file.unlink()
    latest_file.symlink_to(current_file)


def get_data_file_paths(
    base_dir: Path,
    org: str,
    date: datetime,
    data_type: str,
    latest_dir_name: str = "latest",
    create_dirs: bool = True,
) -> Tuple[Path, Path]:
    """
    Get paths for data file and its latest symlink

    Args:
        base_dir: Base directory for data storage
        org: Organization name
        date: Date for the data file
        data_type: Type of data (e.g., 'repos', 'teams', 'members')
        latest_dir_name: Name of the directory for latest symlinks (default: "latest")
        create_dirs: Whether to create the directories if they don't exist (default: True)

    Returns:
        Tuple of (data_file, latest_file)
        - data_file: Path to the date-based data file
        - latest_file: Path to the latest symlink file
    """
    # Get date-based paths
    date_path, latest_dir = get_date_based_paths(
        base_dir=base_dir,
        date=date,
        latest_dir_name=latest_dir_name,
        create_dirs=create_dirs,
    )

    # Generate filenames
    date_str = date.strftime("%Y%m%d")
    filename = f"github_{data_type}_{org}_{date_str}.json"
    latest_filename = f"github_{data_type}_{org}_latest.json"

    return date_path / filename, latest_dir / latest_filename

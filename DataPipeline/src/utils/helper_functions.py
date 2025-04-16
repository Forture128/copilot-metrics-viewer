from pathlib import Path


def ensure_output_dir(output_dir):
    """Ensure the output directory exists and return a Path object.

    Args:
        output_dir (str): Directory path to ensure exists

    Returns:
        Path: Path object of the output directory
    """
    output_path = Path(output_dir)
    output_path.mkdir(parents=True, exist_ok=True)
    return output_path

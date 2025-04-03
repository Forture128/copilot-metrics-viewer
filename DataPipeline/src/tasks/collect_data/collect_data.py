# path: DataPipeline/scripts/collect_data.py
import os
import sys
import json
import asyncio
import aiohttp
from dotenv import load_dotenv

from src.utils.loggers import (
    get_info_logger,
    get_debug_logger,
    get_error_logger,
)

# Add the project root directory to the PYTHONPATH
project_root = os.path.abspath(os.path.join(os.path.dirname(__file__), "../../"))
sys.path.append(project_root)

# Load environment variables from .env file
load_dotenv()

# Configuration
TOKEN = os.getenv("TOKEN")
SCOPE_TYPE = os.getenv("SCOPE_TYPE", "organization")
GITHUB_ORG_NAME = os.getenv("GITHUB_ORG_NAME")
GITHUB_ENT_NAME = os.getenv("GITHUB_ENT_NAME")

# Determine base URL based on scope
if SCOPE_TYPE == "organization":
    BASE_URL = f"https://api.github.com/orgs/{GITHUB_ORG_NAME}"
elif SCOPE_TYPE == "enterprise":
    BASE_URL = f"https://api.github.com/enterprises/{GITHUB_ENT_NAME}"
else:
    raise ValueError("Invalid SCOPE_TYPE. Must be 'organization' or 'enterprise'.")

HEADERS = {
    "Accept": "application/vnd.github+json",
    "Authorization": f"Bearer {TOKEN}",
    "X-GitHub-Api-Version": "2022-11-28",
}
# Set BASE_DIR to the DataPipeline directory
BASE_DIR = os.path.abspath(os.path.join(os.path.dirname(__file__), "../../"))
OUTPUT_DIR = os.path.join(BASE_DIR, "data", "raw")

# Ensure the output directory exists
os.makedirs(OUTPUT_DIR, exist_ok=True)

# Get the loggers
info_logger = get_info_logger(__name__)
error_logger = get_error_logger(__name__)
debug_logger = get_debug_logger(__name__)


async def fetch_data(session, endpoint, output_file):
    """Fetches data from a specified GitHub API endpoint asynchronously and saves it to a JSON file."""
    url = f"{BASE_URL}{endpoint}"
    try:
        async with session.get(url, headers=HEADERS) as response:
            response.raise_for_status()
            data = await response.json()

            # Write the data to a JSON file
            with open(output_file, "w") as f:
                json.dump(data, f, indent=4)

            info_logger.info(f"Data successfully fetched and stored in {output_file}")
    except aiohttp.ClientError as e:
        error_logger.error(f"Error fetching data from {endpoint}: {e}")
        return None


async def fetch_team_data(session, team_tag):
    """Fetches team-specific data asynchronously by replacing placeholders in endpoints."""
    endpoints = {
        f"/team/{team_tag}/copilot/usage": f"{OUTPUT_DIR}/github_team_{team_tag}_metrics.json",
        f"/teams/{team_tag}/members": f"{OUTPUT_DIR}/github_team_{team_tag}_members.json",
    }

    results = await asyncio.gather(
        *(
            fetch_data(session, endpoint, output_file)
            for endpoint, output_file in endpoints.items()
        )
    )
    return results


async def fetch_github_data():
    """Fetches general GitHub data asynchronously, including organization and team data."""
    async with aiohttp.ClientSession() as session:
        # General organization and team data
        endpoints = {
            "/copilot/usage": f"{OUTPUT_DIR}/github_copilot_usage.json",
            "/teams": f"{OUTPUT_DIR}/github_teams.json",
        }

        # Fetch general data
        await asyncio.gather(
            *(
                fetch_data(session, endpoint, output_file)
                for endpoint, output_file in endpoints.items()
            )
        )

        # Load the list of teams from the `github_teams.json` file to fetch team-specific data
        with open(f"{OUTPUT_DIR}/github_teams.json", "r") as f:
            teams = json.load(f)
            team_tags = [team["slug"] for team in teams if "slug" in team]

        if team_tags:
            await asyncio.gather(
                *(fetch_team_data(session, team_tag) for team_tag in team_tags)
            )


if __name__ == "__main__":
    info_logger.info("Starting GitHub data fetch process")
    asyncio.run(fetch_github_data())
    info_logger.info("GitHub data fetch process completed")

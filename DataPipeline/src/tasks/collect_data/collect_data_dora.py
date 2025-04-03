# path: DataPipeline/scripts/collect_data_dora.py
import json
import os
import sys
import aiohttp
import asyncio
from dotenv import load_dotenv

# Add the project root directory to the PYTHONPATH
project_root = os.path.abspath(os.path.join(os.path.dirname(__file__), "../../"))
sys.path.append(project_root)
# from utils.loggers import get_info_logger, get_error_logger, get_debug_logger
from utils.loggers import get_info_logger, get_error_logger, get_debug_logger
from utils.decorators import time_execution

# Load environment variables from .env file
load_dotenv()

# Configuration
TOKEN = os.getenv("TOKEN")
SCOPE_TYPE = os.getenv("SCOPE_TYPE", "organization")
GITHUB_ORG_NAME = os.getenv("GITHUB_ORG_NAME")
GITHUB_ENT_NAME = os.getenv("GITHUB_ENT_NAME")
OWNER_NAME = os.getenv("GITHUB_ORG_NAME")

# Determine base URL based on scope
if SCOPE_TYPE == "organization":
    BASE_URL = f"https://api.github.com/orgs/{GITHUB_ORG_NAME}"
    ORIGIN_BASE_URL = "https://api.github.com"
elif SCOPE_TYPE == "enterprise":
    BASE_URL = f"https://api.github.com/enterprises/{GITHUB_ENT_NAME}"
else:
    raise ValueError("Invalid SCOPE_TYPE. Must be 'organization' or 'enterprise'.")

HEADERS = {
    "Accept": "application/vnd.github+json",
    "Authorization": f"Bearer {TOKEN}",
    "X-GitHub-Api-Version": "2022-11-28",
}
# OUTPUT_DIR = "DataPipeline/data/raw"
# DataPipeline/data/raw
# DataPipeline/scripts/collect_data/collect_data.py
BASE_DIR = os.path.abspath(os.path.join(os.path.dirname(__file__), "../../"))
OUTPUT_DIR = os.path.join(BASE_DIR, "data", "raw/dora")
# Ensure the output directory exists
os.makedirs(OUTPUT_DIR, exist_ok=True)

# Get the loggers
info_logger = get_info_logger(__name__)
error_logger = get_error_logger(__name__)
debug_logger = get_debug_logger(__name__)


@time_execution
async def fetch_data(session, endpoint, output_file):
    """Fetches data from a specified GitHub API endpoint asynchronously and saves it to a JSON file."""
    url = f"{ORIGIN_BASE_URL}{endpoint}"
    print("url = ", url)
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


@time_execution
async def fetch_repository(session, owner, repo):
    """Fetches data for a specific repository."""
    endpoint = f"/repos/{owner}/{repo}"
    output_file = f"{OUTPUT_DIR}/{repo}_repository.json"
    await fetch_data(session, endpoint, output_file)


@time_execution
async def fetch_repositories(session, page=1):
    """Fetches a paginated list of repositories, sorted by update time."""
    endpoint = "/repos"
    params = {"sort": "updated", "per_page": 2, "page": page}
    output_file = f"{OUTPUT_DIR}/repositories_page_{page}.json"
    url = f"{BASE_URL}{endpoint}"
    print("url = ", url)
    # Check if the output file already exists
    if os.path.exists(output_file):
        with open(output_file, "r") as f:
            repositories = json.load(f)
            # load 2 items from the file
            repositories = repositories[:2]
        info_logger.info(f"Data already fetched and stored in {output_file}")
        return repositories[:2]
    try:
        async with session.get(url, headers=HEADERS, params=params) as response:
            response.raise_for_status()
            data = await response.json()

            # Write the data to a JSON file
            with open(output_file, "w") as f:
                json.dump(data, f, indent=4)
            with open(output_file, "r") as f:
                repositories = json.load(f)

            info_logger.info(f"Data successfully fetched and stored in {output_file}")
            return repositories
    except aiohttp.ClientError as e:
        error_logger.error(f"Error fetching repositories page {page}: {e}")


# DORA Metrics Data Fetching Functions
@time_execution
async def fetch_deployment_data(session, owner, repo):
    """Fetches deployment data for a repository."""
    endpoint = f"/repos/{owner}/{repo}/deployments"
    output_file = f"{OUTPUT_DIR}/{repo}_deployments.json"
    await fetch_data(session, endpoint, output_file)


@time_execution
async def fetch_pull_request_data(session, owner, repo):
    """Fetches pull request data for a repository to calculate lead time."""
    endpoint = f"/repos/{owner}/{repo}/pulls"
    output_file = f"{OUTPUT_DIR}/{repo}_pull_requests.json"
    await fetch_data(session, endpoint, output_file)


@time_execution
async def fetch_deployment_status_data(session, owner, repo, deployment_id):
    """Fetches deployment status data for calculating change failure rate."""
    endpoint = f"/repos/{owner}/{repo}/deployments/{deployment_id}/statuses"
    output_file = f"{OUTPUT_DIR}/{repo}_deployment_{deployment_id}_statuses.json"
    await fetch_data(session, endpoint, output_file)


@time_execution
async def fetch_commit_data(session, owner, repo):
    """Fetches commits for calculating lead time for changes."""
    endpoint = f"/repos/{owner}/{repo}/commits"
    output_file = f"{OUTPUT_DIR}/{repo}_commits.json"
    await fetch_data(session, endpoint, output_file)


@time_execution
async def fetch_workflow_run_data(session, owner, repo):
    """Fetches workflow runs for deployment frequency calculation."""
    endpoint = f"/repos/{owner}/{repo}/actions/runs"
    output_file = f"{OUTPUT_DIR}/{repo}_workflow_runs.json"
    await fetch_data(session, endpoint, output_file)


@time_execution
async def fetch_issue_data(session, owner, repo):
    """Fetches issues for calculating MTTR."""
    endpoint = f"/repos/{owner}/{repo}/issues"
    output_file = f"{OUTPUT_DIR}/{repo}_issues.json"
    await fetch_data(session, endpoint, output_file)


@time_execution
async def fetch_dora_metrics(owner):
    """Orchestrates the fetching of all DORA metrics-related data."""
    async with aiohttp.ClientSession() as session:
        repositories = await fetch_repositories(session)
        tasks = []
        for repo in repositories:
            repo_name = repo["name"]
            print("repo_name = ", repo_name)
            tasks.extend(
                [
                    fetch_repository(session, owner, repo_name),
                    fetch_deployment_data(session, owner, repo_name),
                    fetch_pull_request_data(session, owner, repo_name),
                    fetch_commit_data(session, owner, repo_name),
                    fetch_workflow_run_data(session, owner, repo_name),
                    fetch_issue_data(session, owner, repo_name),
                ]
            )
        await asyncio.gather(*tasks)


if __name__ == "__main__":
    info_logger.info("Starting GitHub data fetch process")
    asyncio.run(fetch_dora_metrics(OWNER_NAME))
    info_logger.info("GitHub data fetch process completed")

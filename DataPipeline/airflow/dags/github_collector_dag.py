"""
Airflow DAG for GitHub data collection.

This DAG orchestrates the collection of GitHub data using the GitHubCollector.
It handles data collection, validation, storage, and cleanup with proper error handling
and monitoring.

The DAG follows these best practices:
1. Separation of concerns - business logic in plugins
2. Proper error handling and retries
3. Data validation
4. Configurable through Airflow variables
5. Automatic cleanup of old data
"""

from datetime import timedelta
import logging

from airflow import DAG
from airflow.operators.python import PythonOperator

# from airflow.models import Variable
from airflow.utils.dates import days_ago

from plugins.github_collector_utils import CollectionMode, collect_github_data


# Configure logging
logger = logging.getLogger(__name__)

# DAG configuration
DAG_BASIC_ID = "github_data_collection_basic"
DAG_EXTENDED_ID = "github_data_collection_extended"
DAG_ALL_ID = "github_data_collection_all"

DAG_DESCRIPTION = "Collect and process GitHub repository data"
DAG_SCHEDULE = "@daily"
DAG_TAGS = ["github", "data_collection", "etl"]

# Default arguments
default_args = {
    "owner": "airflow",
    "depends_on_past": False,
    "start_date": days_ago(1),  # Start from yesterday
    "email_on_failure": True,
    "email_on_retry": True,
    "retries": 3,
    "retry_delay": timedelta(minutes=5),
    "execution_timeout": timedelta(hours=1),  # Set timeout
    "sla": timedelta(hours=2),  # Service Level Agreement
}

# Define the DAG
with DAG(
    DAG_BASIC_ID,
    default_args=default_args,
    description=DAG_DESCRIPTION,
    schedule_interval=DAG_SCHEDULE,
    catchup=False,
    tags=DAG_TAGS,
    doc_md=__doc__,  # Use module docstring as DAG documentation
) as dag:
    # Task definitions
    collect_task = PythonOperator(
        task_id="collect_github_data_basic",
        python_callable=collect_github_data,
        provide_context=True,
        op_kwargs={"collection_mode": CollectionMode.BASIC},
        doc_md="""
        Collect GitHub data for the specified organization.
        
        This task:
        1. Collects repository data
        2. Validates the collected data
        3. Saves to timestamped JSON files
        4. Cleans up old files based on retention policy
        
        Configuration is handled through Airflow variables:
        - github_org: Organization to collect data from
        - github_data_dir: Output directory
        - github_concurrent_requests: Number of concurrent API requests
        - github_use_cache: Whether to use caching
        - github_retention_days: Days to keep collected data
        """,
    )

    # Task dependencies
    (collect_task)

# Define the DAG
with DAG(
    DAG_EXTENDED_ID,
    default_args=default_args,
    description=DAG_DESCRIPTION,
    schedule_interval=DAG_SCHEDULE,
    catchup=False,
    tags=DAG_TAGS,
    doc_md=__doc__,  # Use module docstring as DAG documentation
) as dag:
    # Task definitions
    collect_task = PythonOperator(
        task_id="collect_github_data_extended",
        python_callable=collect_github_data,
        provide_context=True,
        op_kwargs={"collection_mode": CollectionMode.EXTENDED},
        doc_md="""
        Collect extended GitHub data for the specified organization.
        
        This task:
        1. Collects extended repository data including:
           - Teams and team members
           - Workflows and workflow runs
           - Repository branches and protection
           - Releases and assets
           - Deployments and deployment statuses
           - Projects and project boards
        2. Validates the collected data
        3. Saves to timestamped JSON files
        4. Cleans up old files based on retention policy
        
        Configuration is handled through Airflow variables:
        - github_org: Organization to collect data from
        - github_data_dir: Output directory
        - github_concurrent_requests: Number of concurrent API requests
        - github_use_cache: Whether to use caching
        - github_retention_days: Days to keep collected data
        
        Collection Mode: extended
        - Collects comprehensive repository metadata
        - Includes team and project information
        - Captures workflow and deployment data
        """,
    )

    # Task dependencies
    (collect_task)

# Define the DAG
with DAG(
    DAG_ALL_ID,
    default_args=default_args,
    description=DAG_DESCRIPTION,
    schedule_interval=DAG_SCHEDULE,
    catchup=False,
    tags=DAG_TAGS,
    doc_md=__doc__,  # Use module docstring as DAG documentation
) as dag:
    # Task definitions
    collect_task = PythonOperator(
        task_id="collect_github_data_all",
        python_callable=collect_github_data,
        provide_context=True,
        op_kwargs={"collection_mode": CollectionMode.ALL},
        doc_md="""
        Collect comprehensive GitHub data for the specified organization.
        
        This task:
        1. Collects all available data including:
           - Basic repository information
           - Extended repository metadata
           - GraphQL-based data
           - Teams and team members
           - Workflows and workflow runs
           - Repository branches and protection
           - Releases and assets
           - Deployments and deployment statuses
           - Projects and project boards
           - Repository collaborators
           - Pull requests and issues
           - Repository languages and topics
           - Security alerts
        2. Validates the collected data
        3. Saves to timestamped JSON files
        4. Cleans up old files based on retention policy
        
        Configuration is handled through Airflow variables:
        - github_org: Organization to collect data from
        - github_data_dir: Output directory
        - github_concurrent_requests: Number of concurrent API requests
        - github_use_cache: Whether to use caching
        - github_retention_days: Days to keep collected data
        
        Collection Mode: all
        - Collects data from all available sources
        - Uses both REST and GraphQL APIs
        - Provides the most comprehensive data collection
        - May take longer to complete due to API rate limits
        """,
    )

    # Task dependencies
    (collect_task)

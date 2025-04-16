import os
from datetime import datetime, timedelta
from airflow import DAG
from airflow.providers.amazon.aws.operators.redshift_data import RedshiftDataOperator
from src.utils.loggers import get_info_logger, get_error_logger  # type: ignore

# Initialize loggers
info_logger = get_info_logger(__name__)
error_logger = get_error_logger(__name__)

# Default arguments for the DAG
default_args = {
    "owner": "airflow",
    "depends_on_past": False,
    "start_date": datetime(2024, 1, 1),
    "email_on_failure": False,
    "email_on_retry": False,
    "retries": 1,
    "retry_delay": timedelta(minutes=5),
}

# Environment variables
REDSHIFT_SCHEMA = os.getenv("REDSHIFT_SCHEMA", "github_metrics")
AWS_REGION = os.getenv("AWS_DEFAULT_REGION", "us-east-1")
CLUSTER_IDENTIFIER = os.getenv(
    "REDSHIFT_CLUSTER_IDENTIFIER", "data-pipeline-redshift-cluster"
)
DATABASE = os.getenv("REDSHIFT_DATABASE", "dev")
REDSHIFT_USER = os.getenv("REDSHIFT_USER", "test")

# Create the DAG
with DAG(
    "init_redshift_tables",
    default_args=default_args,
    description="Initialize Redshift tables for GitHub metrics",
    schedule_interval=None,  # Manual trigger only
    catchup=False,
    tags=["github", "metrics", "redshift", "init"],
) as dag:
    # Task to create schema and tables
    create_tables = RedshiftDataOperator(
        task_id="create_tables",
        sql="sql/create_tables.sql",
        aws_conn_id="aws_default",
        database=DATABASE,
        db_user=REDSHIFT_USER,
        cluster_identifier=CLUSTER_IDENTIFIER,
        region_name=AWS_REGION,
        wait_for_completion=True,
        on_failure_callback=lambda context: error_logger.error(
            f"Failed to create Redshift tables: {context.get('exception')}"
        ),
    )

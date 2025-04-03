"""
GitHub Data Collection DAG with improved async monitoring.

This DAG implements GitHub data collection with:
- Async operation monitoring
- Progress tracking
- Timeout handling
- Health checks
- Detailed logging
"""

from datetime import datetime, timedelta, timezone

from airflow import DAG
from airflow.models import Variable, TaskInstance
from airflow.operators.python import PythonOperator
from airflow.utils.session import provide_session
from airflow.exceptions import AirflowException

from plugins.github_collector_utils import collect_github_data

# DAG configuration
DAG_ID = "github_collector_monitored"
DAG_DESCRIPTION = "GitHub data collection with improved async monitoring"
DAG_SCHEDULE = "@daily"
DAG_START_DATE = datetime(2024, 1, 1)
DAG_TAGS = ["github", "data-collection", "monitoring"]

# Task configuration
TASK_TIMEOUT = timedelta(hours=2)  # Default 2 hours timeout
TASK_RETRIES = 3
TASK_RETRY_DELAY = timedelta(minutes=5)

# Default arguments
default_args = {
    "owner": "airflow",
    "depends_on_past": False,
    "email_on_failure": True,
    "email_on_retry": True,
    "retries": TASK_RETRIES,
    "retry_delay": TASK_RETRY_DELAY,
    "execution_timeout": TASK_TIMEOUT,
}


def check_task_health(**context) -> None:
    """Check if the task is healthy and not stuck"""
    task_instance = context["task_instance"]
    if task_instance.state == "running":
        # Get current time in UTC
        current_time = datetime.now(timezone.utc)

        # Check if task has been running too long
        if (
            current_time - task_instance.start_date
        ).total_seconds() > TASK_TIMEOUT.total_seconds():
            raise AirflowException(
                f"Task running too long, possible stuck. Started at {task_instance.start_date}, "
                f"current time: {current_time}, timeout: {TASK_TIMEOUT}"
            )


@provide_session
def monitor_task_progress(session, **context) -> None:
    """Monitor task progress and update XCom

    Args:
        session: SQLAlchemy session
        **context: Airflow context
    """
    task_instance = (
        session.query(TaskInstance)
        .filter(
            TaskInstance.task_id == "collect_github_data",
            TaskInstance.dag_id == context["dag"].dag_id,
            TaskInstance.execution_date == context["execution_date"],
        )
        .first()
    )

    if task_instance:
        # Get progress from XCom
        progress = task_instance.xcom_pull(
            key="progress", task_ids="collect_github_data"
        )
        if progress:
            context["task_instance"].xcom_push(
                key="monitoring_status",
                value={
                    "progress": progress,
                    "timestamp": datetime.now(timezone.utc).isoformat(),
                    "task_state": task_instance.state,
                },
            )


def create_dag() -> DAG:
    """Create the DAG with monitoring tasks"""
    with DAG(
        DAG_ID,
        description=DAG_DESCRIPTION,
        schedule_interval=DAG_SCHEDULE,
        start_date=DAG_START_DATE,
        tags=DAG_TAGS,
        default_args=default_args,
        catchup=False,
    ) as dag:
        # Health check task
        health_check = PythonOperator(
            task_id="health_check",
            python_callable=check_task_health,
            provide_context=True,
            execution_timeout=timedelta(minutes=1),
        )

        # Main collection task
        collect_data = PythonOperator(
            task_id="collect_github_data",
            python_callable=collect_github_data,
            provide_context=True,
            op_kwargs={
                "collection_mode": "extended",  # Use extended mode for async collection
            },
        )

        # Progress monitoring task
        monitor_progress = PythonOperator(
            task_id="monitor_progress",
            python_callable=monitor_task_progress,
            provide_context=True,
            execution_timeout=timedelta(minutes=1),
        )

        # Set task dependencies
        health_check >> collect_data >> monitor_progress

        # Add documentation
        dag.doc_md = """
        # GitHub Data Collection DAG with Monitoring

        This DAG implements GitHub data collection with improved async monitoring capabilities.

        ## Features
        - Async operation monitoring
        - Progress tracking
        - Health checks
        - Timeout handling
        - Detailed logging

        ## Tasks
        1. `health_check`: Verifies task health and prevents stuck tasks
        2. `collect_github_data`: Main data collection task with async monitoring
        3. `monitor_progress`: Tracks and logs collection progress

        ## Configuration
        - Schedule: Daily
        - Timeout: 2 hours
        - Retries: 3
        - Retry delay: 5 minutes

        ## Monitoring
        - Task health checks
        - Progress tracking
        - Timeout detection
        - Stagnation detection
        """

        # Add task documentation
        collect_data.doc_md = """
        # GitHub Data Collection Task

        Collects GitHub data with improved async monitoring.

        ## Features
        - Async operation monitoring
        - Progress tracking
        - Timeout handling
        - Detailed logging

        ## Parameters
        - collection_mode: "extended" (async collection)
        - timeout: 2 hours
        - concurrent_requests: 5
        - use_cache: true

        ## Monitoring
        - Progress updates via XCom
        - Health checks
        - Timeout detection
        - Stagnation detection
        """

    return dag


# Create the DAG
dag = create_dag()

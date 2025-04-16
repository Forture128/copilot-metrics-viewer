"""
Airflow DAG for GitHub Team and Member Data Collection

This DAG collects GitHub organization data in two parallel streams:
1. Team Data: Collects team structure, members, and repository assignments
2. Member Data: Collects detailed user information and contribution metrics

The data is stored in a date-based directory structure for easy tracking and governance.
"""

from datetime import timedelta

from airflow import DAG
from airflow.operators.bash import BashOperator
from airflow.models import Variable
from airflow.utils.dates import days_ago

# DAG Constants
DAG_ID = "github_team_member_collector"
DAG_DESCRIPTION = "Collect GitHub organization team and member data using CLI"
DAG_SCHEDULE = timedelta(days=1)
DAG_START_DATE = days_ago(1)
DAG_TAGS = ["github", "data_collection"]

# Default arguments
default_args = {
    "owner": "airflow",
    "depends_on_past": False,
    "email_on_failure": False,
    "email_on_retry": False,
    "retries": 1,
    "retry_delay": timedelta(minutes=5),
}

# Output directory
GITHUB_DATA_DIR = "/opt/airflow/data/github"

with DAG(
    DAG_ID,
    default_args=default_args,
    description=DAG_DESCRIPTION,
    schedule_interval=DAG_SCHEDULE,
    start_date=DAG_START_DATE,
    catchup=False,
    tags=DAG_TAGS,
    doc_md=__doc__,
) as dag:
    # Get GitHub organization from Airflow variables
    github_org = Variable.get("github_org", default_var="moneyforward")

    # Create data directory with date-based structure
    data_dir = "{{ execution_date.strftime('/opt/airflow/data/github/%Y/%m/%d') }}"

    create_data_dir = BashOperator(
        task_id="create_data_dir",
        bash_command=f"mkdir -p {data_dir}",
    )

    # Task to collect team data
    collect_teams = BashOperator(
        task_id="collect_team_data",
        bash_command=f"""
        cd /opt/airflow && \
        python -m src.cli.github.collector_cli collect_teams {github_org} \
            --output-dir {data_dir} \
            --include-members \
            --clear-cache
        """,
    )

    # Task to collect member data
    collect_members = BashOperator(
        task_id="collect_member_data",
        bash_command=f"""
        cd /opt/airflow && \
        python -m src.cli.github.collector_cli collect_members {github_org} \
            --output-dir {data_dir} \
            --detailed-info \
            --clear-cache
        """,
    )

    # Create symlinks to latest data files
    create_symlinks = BashOperator(
        task_id="create_symlinks",
        bash_command=f"""
        # Find the latest team and member data files
        TEAM_FILE=$(ls -t {data_dir}/github_teams_*.json | head -1)
        MEMBER_FILE=$(ls -t {data_dir}/github_members_*.json | head -1)
        
        # Create symlinks in the base directory
        ln -sf "$TEAM_FILE" {GITHUB_DATA_DIR}/latest_teams.json
        ln -sf "$MEMBER_FILE" {GITHUB_DATA_DIR}/latest_members.json
        
        echo "Created symlinks to latest team and member data files"
        """,
    )

    # Define task dependencies
    create_data_dir >> [collect_teams, collect_members] >> create_symlinks

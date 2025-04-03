import os
from datetime import datetime, timedelta
import pandas as pd
import awswrangler as wr
from airflow import DAG
from airflow.operators.python import PythonOperator
from utils.loggers import get_info_logger, get_error_logger, get_debug_logger

# Initialize loggers
info_logger = get_info_logger(__name__)
error_logger = get_error_logger(__name__)
debug_logger = get_debug_logger(__name__)

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
DATA_BUCKET = os.getenv("DATA_BUCKET", "github-metrics")
REDSHIFT_CONN = {
    "database": os.getenv("REDSHIFT_DATABASE", "dev"),
    "host": os.getenv("REDSHIFT_HOST", "data-pipeline-localstack"),
    "port": int(os.getenv("REDSHIFT_PORT", "4566")),
    "user": os.getenv("REDSHIFT_USER", "test"),
    "password": os.getenv("REDSHIFT_PASSWORD", "test"),
}
REDSHIFT_SCHEMA = os.getenv("REDSHIFT_SCHEMA", "github_metrics")


def get_s3_path(date_path, file_name):
    """Generate S3 path for data files"""
    path = f"s3://{DATA_BUCKET}/data/{date_path}/{file_name}"
    debug_logger.debug(f"Generated S3 path: {path}")
    return path


def read_s3_data(path, format="json"):
    """Read data from S3 with error handling"""
    debug_logger.debug(f"Reading {format} data from: {path}")
    try:
        if format == "json":
            df = wr.s3.read_json(path)
        elif format == "parquet":
            df = wr.s3.read_parquet(path)
        else:
            raise ValueError(f"Unsupported format: {format}")

        debug_logger.debug(f"Successfully read data. Shape: {df.shape}")
        debug_logger.debug(f"Columns: {df.columns.tolist()}")
        return df
    except Exception as e:
        error_logger.error(f"Error reading {format} data from {path}: {str(e)}")
        raise


def write_s3_data(df, path, format="parquet"):
    """Write data to S3 with error handling"""
    debug_logger.debug(f"Writing {format} data to: {path}")
    debug_logger.debug(f"Data shape: {df.shape}")
    try:
        if format == "parquet":
            wr.s3.to_parquet(df=df, path=path, dataset=True, mode="overwrite")
        else:
            raise ValueError(f"Unsupported format: {format}")
        info_logger.info(f"Successfully wrote data to {path}")
    except Exception as e:
        error_logger.error(f"Error writing {format} data to {path}: {str(e)}")
        raise


def load_to_redshift_table(df, table_name, path):
    """Load data to a Redshift table with error handling"""
    debug_logger.debug(f"Loading data to Redshift table: {table_name}")
    debug_logger.debug(f"Connection details: {REDSHIFT_CONN}")
    debug_logger.debug(f"Data shape: {df.shape}")
    try:
        wr.redshift.copy(
            df=df,
            table=table_name,
            schema=REDSHIFT_SCHEMA,
            con=REDSHIFT_CONN,
            mode="overwrite",
            path=path,
        )
        info_logger.info(f"Successfully loaded data to {REDSHIFT_SCHEMA}.{table_name}")
    except Exception as e:
        error_logger.error(f"Error loading data to {table_name}: {str(e)}")
        raise


def transform_copilot_usage(**context):
    """Transform GitHub Copilot usage data using Pandas"""
    try:
        execution_date = context["execution_date"]
        date_path = execution_date.strftime("%Y/%m/%d")
        info_logger.info(f"Processing Copilot usage data for {date_path}")

        # Read data
        s3_path = get_s3_path(date_path, "github_copilot_usage.json")
        df = read_s3_data(s3_path, "json")

        # Transform data
        debug_logger.debug("Transforming Copilot usage data")
        transformed_df = pd.DataFrame(
            {
                "team_name": df["team_name"],
                "total_hours": df["usage_stats"].apply(lambda x: x.get("total_hours")),
                "active_users": df["usage_stats"].apply(
                    lambda x: x.get("active_users")
                ),
            }
        )
        debug_logger.debug(f"Transformed data shape: {transformed_df.shape}")

        # Save results
        output_path = get_s3_path(date_path, "processed/copilot_usage.parquet")
        write_s3_data(transformed_df, output_path)
        return output_path

    except Exception as e:
        error_logger.error(f"Error in transform_copilot_usage: {str(e)}")
        raise


def transform_team_members(**context):
    """Transform team members data using Pandas"""
    try:
        execution_date = context["execution_date"]
        date_path = execution_date.strftime("%Y/%m/%d")
        info_logger.info(f"Processing team members data for {date_path}")

        # Read data
        s3_path = get_s3_path(date_path, "github_teams.json")
        df = read_s3_data(s3_path, "json")

        # Transform data
        debug_logger.debug("Transforming team members data")
        transformed_df = df.explode("members").reset_index(drop=True)
        transformed_df = pd.DataFrame(
            {
                "team": transformed_df["team"],
                "member_name": transformed_df["members"].apply(lambda x: x.get("name")),
                "member_email": transformed_df["members"].apply(
                    lambda x: x.get("email")
                ),
            }
        )
        debug_logger.debug(f"Transformed data shape: {transformed_df.shape}")

        # Save results
        output_path = get_s3_path(date_path, "processed/team_members.parquet")
        write_s3_data(transformed_df, output_path)
        return output_path

    except Exception as e:
        error_logger.error(f"Error in transform_team_members: {str(e)}")
        raise


def transform_dora_metrics(**context):
    """Transform DORA metrics data using Pandas"""
    try:
        execution_date = context["execution_date"]
        date_path = execution_date.strftime("%Y/%m/%d")
        info_logger.info(f"Processing DORA metrics for {date_path}")

        # Read data
        debug_logger.debug("Reading DORA metrics data")
        deployments_df = read_s3_data(
            get_s3_path(date_path, "dora/*_deployments.json"), "json"
        )
        pr_df = read_s3_data(
            get_s3_path(date_path, "dora/*_pull_requests.json"), "json"
        )
        issues_df = read_s3_data(get_s3_path(date_path, "dora/*_issues.json"), "json")

        # Calculate metrics
        debug_logger.debug("Calculating deployment frequency")
        deployment_frequency = (
            deployments_df.groupby("repository")
            .size()
            .reset_index(name="deployment_frequency")
        )

        debug_logger.debug("Calculating lead time")
        pr_df["created_at"] = pd.to_datetime(pr_df["created_at"])
        pr_df["merged_at"] = pd.to_datetime(pr_df["merged_at"])
        pr_df["lead_time"] = (
            pr_df["merged_at"] - pr_df["created_at"]
        ).dt.total_seconds() / 3600
        lead_time = (
            pr_df.groupby("repository")["lead_time"]
            .mean()
            .reset_index(name="avg_lead_time_hours")
        )

        debug_logger.debug("Calculating MTTR")
        issues_df = issues_df[issues_df["labels"].apply(lambda x: "incident" in x)]
        issues_df["created_at"] = pd.to_datetime(issues_df["created_at"])
        issues_df["closed_at"] = pd.to_datetime(issues_df["closed_at"])
        issues_df["mttr"] = (
            issues_df["closed_at"] - issues_df["created_at"]
        ).dt.total_seconds() / 3600
        mttr = (
            issues_df.groupby("repository")["mttr"]
            .mean()
            .reset_index(name="avg_mttr_hours")
        )

        # Combine metrics
        debug_logger.debug("Combining metrics")
        dora_metrics = deployment_frequency.merge(
            lead_time, on="repository", how="outer"
        ).merge(mttr, on="repository", how="outer")
        debug_logger.debug(f"Final metrics shape: {dora_metrics.shape}")

        # Save results
        output_path = get_s3_path(date_path, "processed/dora_metrics.parquet")
        write_s3_data(dora_metrics, output_path)
        return output_path

    except Exception as e:
        error_logger.error(f"Error in transform_dora_metrics: {str(e)}")
        raise


def load_to_redshift(**context):
    """Load transformed data to Redshift"""
    try:
        execution_date = context["execution_date"]
        date_path = execution_date.strftime("%Y/%m/%d")
        info_logger.info(f"Loading data to Redshift for {date_path}")

        # Load Copilot usage data
        copilot_path = get_s3_path(date_path, "processed/copilot_usage.parquet")
        df = read_s3_data(copilot_path, "parquet")
        load_to_redshift_table(df, "copilot_usage", copilot_path)

        # Load team members data
        team_path = get_s3_path(date_path, "processed/team_members.parquet")
        df = read_s3_data(team_path, "parquet")
        load_to_redshift_table(df, "team_members", team_path)

        # Load DORA metrics data
        dora_path = get_s3_path(date_path, "processed/dora_metrics.parquet")
        df = read_s3_data(dora_path, "parquet")
        load_to_redshift_table(df, "dora_metrics", dora_path)

        info_logger.info("Successfully loaded all data to Redshift")

    except Exception as e:
        error_logger.error(f"Error in load_to_redshift: {str(e)}")
        raise


# Create the DAG
with DAG(
    "github_metrics_pipeline_pandas",
    default_args=default_args,
    description="Transform GitHub metrics data using Pandas and load to Redshift",
    schedule_interval="@daily",
    catchup=False,
    tags=["github", "metrics", "pandas"],
) as dag:
    # Define tasks
    transform_copilot = PythonOperator(
        task_id="transform_copilot_usage",
        python_callable=transform_copilot_usage,
        provide_context=True,
    )

    transform_teams = PythonOperator(
        task_id="transform_team_members",
        python_callable=transform_team_members,
        provide_context=True,
    )

    transform_dora = PythonOperator(
        task_id="transform_dora_metrics",
        python_callable=transform_dora_metrics,
        provide_context=True,
    )

    load_redshift = PythonOperator(
        task_id="load_to_redshift",
        python_callable=load_to_redshift,
        provide_context=True,
    )

    # Set task dependencies
    [transform_copilot, transform_teams, transform_dora] >> load_redshift

from airflow import DAG
from airflow.utils.dates import days_ago
from airflow.providers.apache.spark.operators.spark_submit import SparkSubmitOperator

# from airflow.operators.python import PythonOperator
from airflow.models import Variable
from airflow.hooks.base import BaseHook
from airflow.exceptions import AirflowException
from datetime import timedelta

# Get AWS connection details
aws_conn = BaseHook.get_connection("aws_default")
aws_extra = aws_conn.extra_dejson
endpoint_url = aws_extra.get("host")
region = aws_extra.get("region_name", "us-east-1")

# Define default arguments
default_args = {
    "owner": "airflow",
    "retries": 1,
    "retry_delay": timedelta(minutes=5),
    "max_active_runs": 1,
}

# Get AWS connection details with error handling
try:
    aws_conn = BaseHook.get_connection("aws_default")
    aws_extra = aws_conn.extra_dejson
    endpoint_url = aws_extra.get("host")
    region = aws_extra.get("region_name", "us-east-1")
    print(f"AWS Connection Details: Endpoint URL: {endpoint_url}, Region: {region}")
except Exception as e:
    raise AirflowException(f"AWS Connection Error: {str(e)}")


# Create a reusable Spark task factory
def create_spark_task(task_id, script_path, bucket_name):
    return SparkSubmitOperator(
        task_id=task_id,
        application=script_path,
        conn_id="spark_default",
        verbose=True,
        conf={
            "spark.driver.bindAddress": "0.0.0.0",
            "spark.driver.host": "data-pipeline-airflow",
            "spark.driver.memory": "1g",
            "spark.executor.memory": "1g",
            "spark.network.timeout": "120s",
            "spark.executor.heartbeatInterval": "60s",
            "spark.hadoop.fs.s3a.endpoint": endpoint_url,
            "spark.hadoop.fs.s3a.access.key": aws_conn.login,
            "spark.hadoop.fs.s3a.secret.key": aws_conn.password,
            "spark.hadoop.fs.s3a.path.style.access": "true",
            "spark.hadoop.fs.s3a.impl": "org.apache.hadoop.fs.s3a.S3AFileSystem",
            "spark.hadoop.fs.s3a.aws.credentials.provider": "org.apache.hadoop.fs.s3a.SimpleAWSCredentialsProvider",
            "spark.hadoop.fs.s3a.connection.ssl.enabled": "false",
        },
        application_args=[bucket_name],
    )


with DAG(
    dag_id="dora_metrics_pipeline",
    default_args=default_args,
    schedule_interval="@daily",
    start_date=days_ago(1),
    catchup=False,
) as dag:
    # Fetch bucket name from Airflow Variable
    bucket_name = Variable.get("aws_bucket_name", default_var="test-bucket")

    # Spark tasks
    collect_github_data = create_spark_task(
        "collect_github_data",
        "/opt/airflow/scripts/collect_data/collect_data.py",
        bucket_name,
    )
    collect_dora_data = create_spark_task(
        "collect_dora_data",
        "/opt/airflow/scripts/collect_data/collect_data_dora.py",
        bucket_name,
    )
    upload_to_s3_task = create_spark_task(
        "upload_to_s3", "/opt/airflow/scripts/collect_data/upload_to_s3.py", bucket_name
    )
    transform_data = create_spark_task(
        "transform_data", "/opt/airflow/scripts/transform_data.py", bucket_name
    )

    # Task dependencies
    [collect_github_data, collect_dora_data] >> upload_to_s3_task >> transform_data

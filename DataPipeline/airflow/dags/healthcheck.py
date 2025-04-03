import os
from airflow import DAG
from airflow.operators.python import PythonOperator
from airflow.utils.dates import days_ago
from airflow.hooks.base import BaseHook
import socket
from pyspark.sql import SparkSession
import boto3
import json
import redis
from src.utils.loggers import get_logger


# Create a logger for this module
logger = get_logger("healthcheck")


def create_spark_session():
    """Create a minimal Spark session with S3 configuration"""
    logger.info("Creating Spark session...")

    conf = SparkSession.builder.appName("Test pyspark read json from s3")

    # Basic configurations
    conf = conf.config("spark.driver.host", "data-pipeline-airflow")
    conf = conf.config("spark.driver.bindAddress", "0.0.0.0")

    # S3 Configuration
    conf = conf.config(
        "spark.hadoop.fs.s3a.endpoint", "http://data-pipeline-localstack:4566"
    )
    conf = conf.config("spark.hadoop.fs.s3a.access.key", "test")
    conf = conf.config("spark.hadoop.fs.s3a.secret.key", "test")
    conf = conf.config("spark.hadoop.fs.s3a.path.style.access", "true")
    conf = conf.config(
        "spark.hadoop.fs.s3a.impl", "org.apache.hadoop.fs.s3a.S3AFileSystem"
    )
    conf = conf.config(
        "spark.hadoop.fs.s3a.aws.credentials.provider",
        "org.apache.hadoop.fs.s3a.SimpleAWSCredentialsProvider",
    )

    # Debug configurations
    conf = conf.config("spark.hadoop.fs.s3a.connection.timeout", "120000")
    conf = conf.config("spark.hadoop.fs.s3a.connection.ssl.enabled", "false")
    conf = conf.config("spark.hadoop.fs.s3a.impl.disable.cache", "true")
    conf = conf.config("spark.hadoop.fs.s3a.logging.level", "DEBUG")
    conf = conf.config("spark.executor.heartbeatInterval", "30s")

    # S3A Committer configurations
    conf = conf.config(
        "spark.hadoop.mapreduce.fileoutputcommitter.algorithm.version", "2"
    )
    conf = conf.config(
        "spark.hadoop.mapreduce.fileoutputcommitter.cleanup-failures.ignored", "true"
    )
    conf = conf.config(
        "spark.sql.parquet.output.committer.class",
        "org.apache.spark.internal.io.cloud.BindingParquetOutputCommitter",
    )
    conf = conf.config("spark.hadoop.fs.s3a.committer.name", "directory")
    conf = conf.config("spark.hadoop.fs.s3a.committer.staging.conflict-mode", "replace")
    conf = conf.config(
        "spark.hadoop.fs.s3a.committer.staging.tmp.path", "/tmp/spark_staging"
    )
    conf = conf.config("spark.hadoop.fs.s3a.buffer.dir", "/tmp/spark_buffer")
    conf = conf.config("spark.hadoop.fs.s3a.fast.upload", "true")
    conf = conf.config("spark.hadoop.fs.s3a.fast.upload.buffer", "disk")
    conf = conf.config("spark.hadoop.fs.s3a.multipart.size", "5242880")  # 5MB
    conf = conf.config("spark.hadoop.fs.s3a.connection.maximum", "100")

    # Config HIVE
    conf = conf.config("spark.sql.catalogImplementation", "in-memory")

    session = conf.getOrCreate()

    # Enable detailed logging
    log4j = session._jvm.org.apache.log4j
    log4j.Logger.getLogger("org.apache.hadoop.fs.s3a").setLevel(log4j.Level.DEBUG)
    log4j.Logger.getLogger("org.apache.spark.internal.io.cloud").setLevel(
        log4j.Level.DEBUG
    )

    return session


def check_spark_connection():
    """Check if Spark master is accessible"""
    try:
        # Try to connect to Spark master
        sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        sock.settimeout(5)
        result = sock.connect_ex(("data-pipeline-spark-master", 7077))
        if result != 0:
            raise Exception("Cannot connect to Spark master")
        sock.close()
        return True
    except Exception as e:
        raise Exception(f"Spark connection failed: {str(e)}")


def check_localstack_connection():
    """Check if LocalStack is accessible and S3 is working"""
    try:
        # Get the AWS connection details
        aws_conn = BaseHook.get_connection("aws_default")
        endpoint_url = (
            f"http://{aws_conn.host}:{aws_conn.port}"
            if aws_conn.port
            else f"http://{aws_conn.host}"
        )
        region = aws_conn.extra_dejson.get("region_name", "us-east-1")

        logger.info(f"Connecting to LocalStack at {endpoint_url}")

        # Create a boto3 client
        s3 = boto3.client(
            "s3",
            endpoint_url=endpoint_url,
            aws_access_key_id=aws_conn.login,
            aws_secret_access_key=aws_conn.password,
            region_name=region,
            verify=False,  # Skip SSL verification for LocalStack
        )

        # Try to list buckets
        response = s3.list_buckets()
        logger.info(f"Successfully connected to LocalStack S3. Buckets: {response}")
        return True
    except Exception as e:
        logger.error(
            f"LocalStack connection details - endpoint: {endpoint_url}, region: {region}"
        )
        raise Exception(f"LocalStack S3 connection failed: {str(e)}")


def check_localstack_upload_file_to_s3():
    """Check if we can upload file to s3"""
    try:
        aws_conn = BaseHook.get_connection("aws_default")
        endpoint_url = f"http://{aws_conn.host}:{aws_conn.port}"
        region = aws_conn.extra_dejson.get("region_name", "us-east-1")

        s3 = boto3.client(
            "s3",
            endpoint_url=endpoint_url,
            aws_access_key_id=aws_conn.login,
            aws_secret_access_key=aws_conn.password,
            region_name=region,
            verify=False,
        )
        # Create test.json
        with open("test.json", "w", encoding="utf-8") as f:
            f.write(
                """
                {
                "day": "2025-01-09",
                "total_suggestions_count": 1000,
                "total_acceptances_count": 800,
                "total_lines_suggested": 5000,
                "total_lines_accepted": 4000,
                "total_active_users": 100,
                "total_chat_acceptances": 200,
                "total_chat_turns": 500,
                "total_active_chat_users": 50,
                "breakdown": [
                    {
                    "language": "python",
                    "suggestions_count": 500,
                    "acceptances_count": 400
                    },
                    {
                    "language": "javascript",
                    "suggestions_count": 500,
                    "acceptances_count": 400
                    }
                ]
                }
                """
            )

        s3.upload_file(
            "test.json",
            "test-bucket",
            "data/2025/01/09/test.json",
        )

        logger.info("Successfully uploaded file to s3")
        return True
    except Exception as e:
        raise Exception(f"Upload file to s3 failed: {str(e)}")


def check_download_json_from_s3_localstack():
    """Check if we can download json from s3"""
    try:
        aws_conn = BaseHook.get_connection("aws_default")
        endpoint_url = f"http://{aws_conn.host}:{aws_conn.port}"
        region = aws_conn.extra_dejson.get("region_name", "us-east-1")

        logger.info(f"Connecting to LocalStack S3 at {endpoint_url}")

        s3 = boto3.client(
            "s3",
            endpoint_url=endpoint_url,
            aws_access_key_id=aws_conn.login,
            aws_secret_access_key=aws_conn.password,
            region_name=region,
            verify=False,
        )

        # First check if bucket exists
        try:
            s3.head_bucket(Bucket="test-bucket")
            logger.info("Successfully connected to test-bucket")
        except Exception as e:
            raise Exception(f"Bucket test-bucket not found or not accessible: {str(e)}")

        # Check if file exists
        try:
            s3.head_object(Bucket="test-bucket", Key="data/2025/01/09/test.json")
            logger.info("Test file exists in S3")
        except Exception as e:
            raise Exception(f"Test file not found in S3: {str(e)}")

        # Download the file
        logger.info("Attempting to download file...")
        s3.download_file(
            "test-bucket",
            "data/2025/01/09/test.json",
            "test.json",
        )

        # Verify downloaded file
        if not os.path.exists("test.json"):
            raise Exception("File was not downloaded successfully")

        # Read and validate JSON content
        with open("test.json", "r") as f:
            data = json.loads(f.read())
            required_fields = ["day", "total_suggestions_count", "breakdown"]
            missing_fields = [field for field in required_fields if field not in data]
            if missing_fields:
                raise Exception(
                    f"Downloaded JSON is missing required fields: {missing_fields}"
                )

        logger.info("Successfully downloaded and validated JSON file")
        return True

    except Exception as e:
        logger.error(f"Error details: {str(e)}")
        logger.error(f"Connection details - endpoint: {endpoint_url}, region: {region}")
        raise Exception(f"Download json from s3 failed: {str(e)}")


def test_spark_execution_local():
    """Test spark execution"""
    try:
        spark = SparkSession.builder.appName("HealthCheck").getOrCreate()
        df = (
            spark.read.format("json")
            .option("multiLine", "true")  # Handle multi-line JSON
            .option("inferSchema", "true")  # Automatically infer schema
            .json("github_copilot_usage.json")
        )

        # Print schema and sample data
        logger.info("\nSchema:")
        df.printSchema()

        logger.info(f"\nTotal records: {df.count()}")

        logger.info("\nComplete data:")
        df.show(n=1000, truncate=False)  # Show all rows, don't truncate

        # Analyze breakdown data
        logger.info("\nBreakdown Analysis:")
        breakdown_df = df.select("breakdown").first()[0]
        logger.info(f"Number of language breakdowns: {len(breakdown_df)}")

        # Print summary statistics
        logger.info("\nSummary Statistics:")
        numeric_columns = [
            "total_suggestions_count",
            "total_acceptances_count",
            "total_lines_suggested",
            "total_lines_accepted",
            "total_active_users",
            "total_chat_acceptances",
            "total_chat_turns",
            "total_active_chat_users",
        ]
        df.select(numeric_columns).describe().show(truncate=False)

        # Check if we have the expected columns
        expected_columns = [
            "day",
            "total_suggestions_count",
            "total_acceptances_count",
            "total_lines_suggested",
            "total_lines_accepted",
            "total_active_users",
            "total_chat_acceptances",
            "total_chat_turns",
            "total_active_chat_users",
            "breakdown",
        ]

        missing_columns = [col for col in expected_columns if col not in df.columns]
        if missing_columns:
            raise Exception(f"Missing expected columns: {missing_columns}")

        return True
    except Exception as e:
        raise Exception(f"Test spark execution failed: {str(e)}")


def test_spark_execution_localstack():
    """Test spark execution"""
    try:
        spark = create_spark_session()
        df = (
            spark.read.format("json")
            .option("multiLine", "true")  # Handle multi-line JSON
            .option("inferSchema", "true")  # Automatically infer schema
            .json("s3a://test-bucket/data/2025/01/09/github_copilot_usage.json")
        )

        # Print schema and sample data
        logger.info("\nSchema:")
        df.printSchema()

        logger.info(f"\nTotal records: {df.count()}")

        logger.info("\nComplete data:")
        df.show(n=1000, truncate=False)  # Show all rows, don't truncate

        # Analyze breakdown data
        logger.info("\nBreakdown Analysis:")
        breakdown_df = df.select("breakdown").first()[0]
        logger.info(f"Number of language breakdowns: {len(breakdown_df)}")

        # Print summary statistics
        logger.info("\nSummary Statistics:")
        numeric_columns = [
            "total_suggestions_count",
            "total_acceptances_count",
            "total_lines_suggested",
            "total_lines_accepted",
            "total_active_users",
            "total_chat_acceptances",
            "total_chat_turns",
            "total_active_chat_users",
        ]
        df.select(numeric_columns).describe().show(truncate=False)

        # Check if we have the expected columns
        expected_columns = [
            "day",
            "total_suggestions_count",
            "total_acceptances_count",
            "total_lines_suggested",
            "total_lines_accepted",
            "total_active_users",
            "total_chat_acceptances",
            "total_chat_turns",
            "total_active_chat_users",
            "breakdown",
        ]

        missing_columns = [col for col in expected_columns if col not in df.columns]
        if missing_columns:
            raise Exception(f"Missing expected columns: {missing_columns}")

        return True
    except Exception as e:
        raise Exception(f"Test spark execution failed: {str(e)}")


def check_kafka_connection():
    """Check if Kafka is accessible"""
    try:
        # Try to connect to Kafka
        sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        sock.settimeout(5)
        result = sock.connect_ex(("kafka", 9092))
        if result != 0:
            raise Exception("Cannot connect to Kafka")
        sock.close()
        return True
    except Exception as e:
        raise Exception(f"Kafka connection failed: {str(e)}")


def check_zookeeper_connection():
    """Check if Zookeeper is accessible"""
    try:
        # Try to connect to Zookeeper
        sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        sock.settimeout(5)
        result = sock.connect_ex(("zookeeper", 2181))
        if result != 0:
            raise Exception("Cannot connect to Zookeeper")
        sock.close()
        return True
    except Exception as e:
        raise Exception(f"Zookeeper connection failed: {str(e)}")


def check_redis_connection():
    """Check if Redis is accessible"""
    try:
        # Try to connect to Redis
        sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        sock.settimeout(5)
        result = sock.connect_ex(("redis", 6379))
        if result != 0:
            raise Exception("Cannot connect to Redis")
        sock.close()

        # Try to create a Redis client and perform a simple operation
        redis_client = redis.Redis(host="redis", port=6379, db=0, socket_timeout=5)
        redis_client.ping()  # Simple ping to check if Redis is responding

        logger.info("Successfully connected to Redis")
        return True
    except Exception as e:
        raise Exception(f"Redis connection failed: {str(e)}")


default_args = {
    "owner": "airflow",
    "depends_on_past": False,
    "start_date": days_ago(1),
    "email_on_failure": False,
    "email_on_retry": False,
}

with DAG(
    "service_healthcheck",
    default_args=default_args,
    description="Check health of all required services",
    schedule_interval="*/5 * * * *",  # Run every 5 minutes
    catchup=False,
    tags=["healthcheck", "interval"],
) as dag:
    # Check Spark connection
    check_spark = PythonOperator(
        task_id="check_spark", python_callable=check_spark_connection
    )

    # Check LocalStack/S3 connection
    check_localstack = PythonOperator(
        task_id="check_localstack", python_callable=check_localstack_connection
    )

    # Check LocalStack/S3 upload file
    check_localstack_upload_file = PythonOperator(
        task_id="check_localstack_upload_file",
        python_callable=check_localstack_upload_file_to_s3,
    )

    # Check download json from s3
    check_download_json_from_s3 = PythonOperator(
        task_id="check_download_json_from_s3",
        python_callable=check_download_json_from_s3_localstack,
    )

    # Check Kafka connection
    check_kafka = PythonOperator(
        task_id="check_kafka", python_callable=check_kafka_connection
    )

    # Check Zookeeper connection
    check_zookeeper = PythonOperator(
        task_id="check_zookeeper", python_callable=check_zookeeper_connection
    )

    # Check Redis connection
    check_redis = PythonOperator(
        task_id="check_redis", python_callable=check_redis_connection
    )

    # Check spark execution in local airflow
    check_spark_execution = PythonOperator(
        task_id="check_spark_execution", python_callable=test_spark_execution_local
    )
    # Check spark execution in localstack
    check_spark_execution_localstack = PythonOperator(
        task_id="check_spark_execution_localstack",
        python_callable=test_spark_execution_localstack,
    )

    # Set up dependencies
    (
        [
            check_spark,
            check_localstack,
            check_localstack_upload_file,
            check_download_json_from_s3,
            check_kafka,
            check_zookeeper,
            check_redis,
            check_spark_execution,
            check_spark_execution_localstack,
        ]
    )

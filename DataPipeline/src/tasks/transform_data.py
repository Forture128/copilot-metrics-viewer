import os
import sys
from datetime import datetime
from pyspark.sql import SparkSession
from pyspark.sql.functions import col, explode, avg, count, lit
import logging

# Configure logging
logging.basicConfig(
    level=logging.INFO, format="%(asctime)s - %(levelname)s - %(message)s"
)
logger = logging.getLogger(__name__)


def get_s3_path(bucket_name, date=None):
    """Generate S3 path with date"""
    if date is None:
        date = datetime.now().strftime("%Y/%m/%d")
    return f"s3a://{bucket_name}/data/{date}"


def create_spark_session():
    """Create a minimal Spark session with S3 configuration"""
    logger.info("Creating Spark session...")

    conf = SparkSession.builder.appName("Data Transformation Pipeline")

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

    # Enable detailed logging for S3A
    log4j = session._jvm.org.apache.log4j
    log4j.Logger.getLogger("org.apache.hadoop.fs.s3a").setLevel(log4j.Level.DEBUG)

    return session


def load_raw_data(spark, bucket_name, file_pattern):
    """Load raw JSON data from S3"""
    s3_base_path = get_s3_path(bucket_name)
    file_path = f"{s3_base_path}/{file_pattern}"
    logger.info(f"Loading data from {file_path}")

    try:
        df = (
            spark.read.format("json")
            .option("multiLine", "true")
            .option("inferSchema", "true")
            .json(file_path)
        )
        # Print schema and sample data
        logger.info("\nSchema:")
        df.printSchema()
        logger.info(f"\nTotal records: {df.count()}")

        logger.info(f"Successfully loaded {df.count()} records")
        df.printSchema()
        return df

    except Exception as e:
        logger.error(f"Error loading data: {str(e)}", exc_info=True)
        return None


def save_transformed_data(dataframe, bucket_name, output_file):
    """Save transformed data to S3 in Parquet format"""
    s3_output_path = f"{get_s3_path(bucket_name)}/processed/{output_file}"
    try:
        logger.info(f"Saving data to {s3_output_path}")
        dataframe.write.mode("overwrite").parquet(s3_output_path)
        logger.info("Save completed successfully")
    except Exception as e:
        logger.error(f"Error saving data: {str(e)}", exc_info=True)
        raise


def transform_team_members(dataframe):
    """Transform team members data with error handling for missing fields"""
    logger.info("Transforming team members data")
    try:
        # Select relevant fields from the team data
        transformed = dataframe.select(
            col("name").alias("team_name"),
            col("id").alias("team_id"),
            col("slug"),
            col("description"),
            col("privacy"),
            col("parent.name").alias("parent_team_name"),
            col("parent.id").alias("parent_team_id"),
        )
        return transformed
    except Exception as e:
        logger.error(f"Error: {e}", exc_info=True)
        return None


def transform_copilot_usage(dataframe):
    """Transform Copilot usage data with breakdown"""
    logger.info("Transforming Copilot usage data")
    try:
        transformed = dataframe.withColumn(
            "breakdown", explode(col("breakdown"))
        ).select(
            col("day"),
            col("breakdown.language"),
            col("breakdown.suggestions_count"),
            col("breakdown.acceptances_count"),
            col("breakdown.active_users"),
        )
        return transformed
    except Exception as e:
        logger.error(f"Error: {e}", exc_info=True)
        return None


def transform_dora_metrics(spark, bucket_name):
    """Transform DORA metrics data from S3"""
    try:
        logger.info("Transforming DORA metrics")
        s3_base_path = get_s3_path(bucket_name)

        # Load data
        deployments_df = spark.read.json(f"{s3_base_path}/dora/*_deployments.json")
        pull_requests_df = spark.read.json(f"{s3_base_path}/dora/*_pull_requests.json")
        issues_df = spark.read.json(f"{s3_base_path}/dora/*_issues.json")

        # Calculate metrics
        deployment_frequency = (
            deployments_df.groupBy("repository")
            .count()
            .withColumnRenamed("count", "deployment_frequency")
        )

        lead_time = (
            pull_requests_df.withColumn(
                "lead_time",
                (
                    col("merged_at").cast("timestamp").cast("long")
                    - col("created_at").cast("timestamp").cast("long")
                )
                / 3600,
            )
            .groupBy("repository")
            .agg(avg("lead_time").alias("avg_lead_time_hours"))
        )

        mttr = (
            issues_df.filter(col("labels").contains("incident"))
            .withColumn(
                "mttr_hours",
                (
                    col("closed_at").cast("timestamp").cast("long")
                    - col("created_at").cast("timestamp").cast("long")
                )
                / 3600,
            )
            .groupBy("repository")
            .agg(avg("mttr_hours").alias("avg_mttr_hours"))
        )

        # Combine metrics
        dora_metrics = deployment_frequency.join(lead_time, "repository", "outer").join(
            mttr, "repository", "outer"
        )

        # Save results
        save_transformed_data(dora_metrics, bucket_name, "dora_metrics.parquet")
        logger.info("DORA metrics transformation completed")

    except Exception as e:
        logger.error(f"Error in DORA metrics transformation: {str(e)}", exc_info=True)
        raise


def main(bucket_name):
    logger.info(f"Starting data transformation pipeline for bucket: {bucket_name}")
    spark = create_spark_session()

    try:
        # Process Copilot usage data
        if copilot_df := load_raw_data(spark, bucket_name, "github_copilot_usage.json"):
            transformed_copilot = transform_copilot_usage(copilot_df)
            save_transformed_data(
                transformed_copilot, bucket_name, "copilot_usage.parquet"
            )

        # Process team members data
        if team_df := load_raw_data(spark, bucket_name, "github_teams.json"):
            transformed_team = transform_team_members(team_df)
            save_transformed_data(transformed_team, bucket_name, "team_members.parquet")

        # Transform DORA metrics
        transform_dora_metrics(spark, bucket_name)

        logger.info("Data transformation pipeline completed successfully")

    except Exception as e:
        logger.error(f"Pipeline failed: {str(e)}", exc_info=True)
        sys.exit(1)
    finally:
        spark.stop()


if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("Usage: transform_data.py <bucket_name>")
        sys.exit(1)

    bucket_name = sys.argv[1]
    main(bucket_name)

import os
import boto3
from datetime import datetime
from botocore.exceptions import ClientError
import glob
from airflow.hooks.base import BaseHook


def upload_to_s3(local_dir, bucket_name, s3_prefix=None):
    """
    Upload all JSON files from local_dir to S3 with date-based prefixes
    Returns a list of uploaded S3 paths
    """
    # Get the AWS connection details
    aws_conn = BaseHook.get_connection("aws_default")
    endpoint_url = (
        f"http://{aws_conn.host}:{aws_conn.port}"
        if aws_conn.port
        else f"http://{aws_conn.host}"
    )
    region = aws_conn.extra_dejson.get("region_name", "us-east-1")

    print(f"Connecting to LocalStack at {endpoint_url}")
    s3_client = boto3.client(
        "s3",
        endpoint_url=endpoint_url,
        aws_access_key_id=aws_conn.login,
        aws_secret_access_key=aws_conn.password,
        region_name=region,
        verify=False,
    )

    uploaded_paths = []
    current_date = datetime.now().strftime("%Y/%m/%d")

    # Walk through all files in the directory
    for root, _, files in os.walk(local_dir):
        for file in files:
            if file.endswith(".json"):
                local_path = os.path.join(root, file)

                # Create S3 key with date-based prefix
                relative_path = os.path.relpath(local_path, local_dir)
                if s3_prefix:
                    s3_key = f"{s3_prefix}/{current_date}/{relative_path}"
                else:
                    s3_key = f"data/{current_date}/{relative_path}"

                try:
                    s3_client.upload_file(local_path, bucket_name, s3_key)
                    uploaded_paths.append(f"s3://{bucket_name}/{s3_key}")
                    print(f"Uploaded {local_path} to s3://{bucket_name}/{s3_key}")
                except ClientError as e:
                    print(f"Error uploading {local_path}: {e}")
                    continue

                # Delete the local file after successful upload
                os.remove(local_path)

    return uploaded_paths


def clean_directory(directory):
    """Remove all files in the specified directory"""
    try:
        for file in glob.glob(os.path.join(directory, "**/*.json"), recursive=True):
            os.remove(file)
        print(f"Cleaned directory: {directory}")
    except Exception as e:
        print(f"Error cleaning directory {directory}: {e}")


if __name__ == "__main__":
    # Example usage
    base_dir = os.path.abspath(os.path.join(os.path.dirname(__file__), "../../"))
    print(f"Base dir: {base_dir}")
    raw_data_dir = os.path.join(base_dir, "data", "raw")
    print(f"Raw data dir: {raw_data_dir}")
    bucket_name = "test-bucket"

    upload_to_s3(raw_data_dir, bucket_name)
    clean_directory(raw_data_dir)

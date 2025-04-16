#!/bin/bash
set -e

echo "Running LocalStack initialization script..."

# Wait for LocalStack to be ready
echo "Waiting for LocalStack to be ready..."
sleep 10

# Create S3 bucket test-bucket
echo "Creating S3 bucket: test-bucket"
awslocal s3 mb s3://test-bucket
echo "S3 bucket created successfully"

# List buckets for confirmation
echo "Listing S3 buckets:"
awslocal s3 ls

# # Create Redshift cluster (commented out for now)
# echo "Creating Redshift cluster..."
# awslocal redshift create-cluster --cluster-identifier data-pipeline-redshift-cluster --node-type dc2.large --master-username test --master-user-password test --db-name dev
# echo "Redshift cluster creation initiated"

echo "LocalStack initialization completed successfully"


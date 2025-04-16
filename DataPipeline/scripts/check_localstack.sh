#!/bin/bash

# Check if LocalStack is running
echo "Checking LocalStack status..."

# Check the health endpoint
HEALTH_RESPONSE=$(curl -s http://localhost:4566/_localstack/health)
echo "Health response: $HEALTH_RESPONSE"

# Check if the S3 bucket was created
echo "Checking S3 buckets..."
AWS_ENDPOINT=http://localhost:4566 aws --endpoint-url=http://localhost:4566 s3 ls
echo ""

# Check the container logs
echo "Checking LocalStack container logs..."
docker logs data-pipeline-localstack | grep -i "initialization script" || echo "No initialization script logs found"
echo ""

# Check if initialization script exists in container
echo "Checking if script exists in container..."
docker exec data-pipeline-localstack ls -la /docker-entrypoint-initaws.d/ || echo "Directory not found" 
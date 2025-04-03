# Create S3 bucket test-bucket
awslocal s3 mb s3://test-bucket

# Create dummy JSON file
cat << EOF > dummy_data.json
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
EOF

# Upload dummy data to S3
awslocal s3 cp dummy_data.json s3://test-bucket/data/2025/01/09/github_copilot_usage.json

# Create Redshift cluster
# awslocal redshift create-cluster --cluster-identifier data-pipeline-redshift-cluster --node-type dc2.large --master-username test --master-user-password test --db-name dev


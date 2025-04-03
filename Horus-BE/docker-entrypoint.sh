#!/bin/bash
set -e

# Wait for the database to be ready
echo "Waiting for database to be ready..."
./wait-for-it.sh ${DATABASE_HOST:-postgres}:${DATABASE_PORT:-5432} -t 60

# Run migrations
echo "Running database migrations..."
diesel migration run

# Check if SETUP_ADMIN environment variable is set to true
if [ "${SETUP_ADMIN}" == "true" ]; then
  echo "Setting up admin account..."
  cargo run --bin seed_admin
fi

# Start the application
echo "Starting application..."
exec "$@" 
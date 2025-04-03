#!/bin/bash

# Initialize the database
airflow db init

# Create admin user if not exists
airflow users create \
    --username admin \
    --firstname admin \
    --lastname admin \
    --role Admin \
    --email admin@example.com \
    --password admin \
    --skip-if-exists

# Start the scheduler in the background
airflow scheduler &

# Start the webserver
exec airflow webserver 
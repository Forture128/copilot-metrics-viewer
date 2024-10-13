#!/bin/bash

# Create directories
mkdir -p data/raw
mkdir -p data/processed
mkdir -p data/analysis
mkdir -p notebooks
mkdir -p scripts
mkdir -p docker/config
mkdir -p airflow/dags
mkdir -p airflow/config
mkdir -p k8s/manifests
mkdir -p logs
mkdir -p docs

# Create files
touch notebooks/data_ingestion.ipynb
touch notebooks/data_processing.ipynb
touch notebooks/visualization.ipynb
touch scripts/collect_data.py
touch scripts/transform_data.py
touch scripts/visualize_data.py
touch docker/Dockerfile
touch docker/docker-compose.yml
touch logs/pipeline.log
touch Pipfile
touch Pipfile.lock
touch README.md
touch docs/pipeline_overview.md
touch docs/ci_cd.md
touch docs/data_model.md
touch docs/jupyter_integration.md

echo "Folder structure and files created successfully."
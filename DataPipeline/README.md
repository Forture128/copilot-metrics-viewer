## Tools and Technologies
- **Data Collection**: Kafka, REST APIs, Webhooks
- **Data Processing**: PySpark, Pandas, Dask
- **Data Storage**: S3, Redshift, DynamoDB
- **Data Visualization**: Matplotlib, Seaborn, Plotly
- **Data Orchestration**: Airflow, Prefect, Dagster

- **Containerization**: Docker, Kubernetes, Helm
- **CI/CD**: GitHub Actions, Jenkins, CircleCI
- **Monitoring**: Prometheus, Grafana, ELK Stack

- **Testing**: Pytest, Unittest, Mocha, Jest
- **Logging**: ELK Stack, Fluentd, Logstash
- **Security**: Vault, AWS KMS, OAuth2

- **Formatting**: Black, Prettier, ESLint, Ruff

## Folder Structure
```bash
data-pipeline/
├── data/
│   ├── raw/
│   ├── processed/
│   └── analysis/
├── notebooks/
│   ├── data_ingestion.ipynb
│   ├── data_processing.ipynb
│   └── visualization.ipynb
├── scripts/
│   ├── collect_data.py
│   ├── transform_data.py
│   └── visualize_data.py
├── docker/
│   ├── Dockerfile
│   ├── docker-compose.yml
│   └── config/
├── airflow/
│   ├── dags/
│   └── config/
├── k8s/
│   └── manifests/
├── logs/
│   └── pipeline.log
├── Pipfile
├── Pipfile.lock
├── README.md
└── docs/
    ├── pipeline_overview.md
    ├── ci_cd.md
    ├── data_model.md
    └── jupyter_integration.md
```

## Folder Descriptions

1. **data/**: Directory for storing raw and processed datasets.
    - **raw/**: Store raw data ingested from APIs.
    - **processed/**: Store processed and transformed data.
    - **analysis/**: Store data analysis results.
2. **notebooks/**: Jupyter notebooks for experimentation and prototyping.
    - **data_ingestion.ipynb**: For ingesting data from APIs.
    - **data_processing.ipynb**: For data transformations with PySpark.
    - **visualization.ipynb**: For data visualizations using Matplotlib, Seaborn, etc.
3. **scripts/**: Python scripts for various pipeline tasks.
    - **collect_data.py**: Script for data collection from Kafka streams and batch jobs.
    - **transform_data.py**: For transforming raw data using PySpark.
    - **visualize_data.py**: For generating visualizations of data.
4. **docker/**: Docker-related configurations.
    - **Dockerfile**: Define the environment for the pipeline.
    - **docker-compose.yml**: Compose services like Kafka, Airflow, Jupyter.
5. **airflow/**: Airflow DAGs and configuration for batch orchestration.
6. **k8s/**: Kubernetes manifests for deployment (EKS).
7. **logs/**: Log directory to store pipeline logs for debugging and monitoring.
8. **Pipfile & Pipfile.lock**: Define the virtual environment dependencies using Pipenv.
9. **docs/**: Documentation files.
    - **pipeline_overview.md**: Explains the overall architecture and flow.
    - **ci_cd.md**: Details of the CI/CD pipeline and deployment.
    - **data_model.md**: Data model definitions for GitHub, Jira, Slack metrics.
    - **jupyter_integration.md**: Instructions for integrating Jupyter Notebooks into the pipeline.


## Install (Mac OS)

```bash

# Install pyenv
# (https://github.com/pyenv/pyenv)
$ pyenv install $(cat .python-version)
$ pyenv rehash


# It is useful to register python3 alias
$ echo 'alias python=python3' >> ~/.zshrc
$ source ~/.zshrc

# Install pipenv
$ pip install pipenv

# Specify that the virtual environment should be created in backend/venv
$ echo 'export PIPENV_VENV_IN_PROJECT=1' >> ~/.zshrc
$ source ~/.zshrc
```

## Setting up Kubernetes Locally with Helm and Minikube

1. Install Minikube: Follow the instructions here to install Minikube for your OS.

2. Start Minikube: Run minikube start to start the Minikube cluster.
```bash
$ minikube start
```

3. Install Helm: Follow the instructions here to install Helm for your OS.
Helm is used to manage Kubernetes resources. Install Helm on your local machine:
```bash
brew install helm # for Mac
```
4. Create Helm Charts: Create Helm charts for the services you want to deploy.
You can create your Helm chart for your application (e.g., Jupyter, Kafka, Localstack, etc.). Here’s a basic structure:
```bash
helm create data-pipeline
```
This command generates a chart template. You can modify it according to your needs (e.g., define deployments for Jupyter, Localstack, etc.).

In the data-pipeline chart, modify the values.yaml to deploy Localstack as part of the Kubernetes cluster:
Example Helm Chart for Localstack:
```bash
# values.yaml
localstack:
  image: localstack/localstack
  tag: latest
  service:
    type: ClusterIP
    port: 4566
```
Then, define the Kubernetes deployment for Localstack in the templates/deployment.yaml file:
```bash
# templates/deployment.yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: localstack
spec:
  replicas: 1
  selector:
    matchLabels:
      app: localstack
  template:
    metadata:
      labels:
        app: localstack
    spec:
      containers:
        - name: localstack
          image: "{{ .Values.localstack.image }}:{{ .Values.localstack.tag }}"
          ports:
            - containerPort: 4566
          env:
            - name: SERVICES
              value: "s3,redshift,lambda"
            - name: DEBUG
              value: "1"
```
5. Deploy Helm Charts: Deploy the Helm charts to the Minikube cluster.
```bash
helm install data-pipeline ./data-pipeline
```
This will deploy Localstack (and any other services you define) into your local Minikube cluster.

### Example Workflow with Kubernetes and Helm Locally:

1.	Run Minikube: Start Minikube to launch your local Kubernetes cluster.

2.	Install Localstack: Use Helm to install Localstack in your cluster.

3.	Deploy Kafka: Add Kafka, Jupyter, and Airflow services to your Helm chart and deploy them locally.

4.	Develop and Test: Use Localstack to mock AWS services while developing and testing the pipeline locally.

5.	Move to Cloud: Once the development is done, you can use the same Helm chart for deploying on cloud environments like AWS (EKS).
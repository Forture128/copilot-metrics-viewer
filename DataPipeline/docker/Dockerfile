# Use the official Airflow image as the base image
FROM apache/airflow:2.10.4

USER root

# Install Java for PySpark
RUN apt-get update && \
    apt-get install -y default-jdk curl && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# Set environment variables for Java and Spark
ENV JAVA_HOME=/usr/lib/jvm/java-17-openjdk-arm64
ENV PATH=$JAVA_HOME/bin:$PATH

# Install Spark
RUN curl -O https://archive.apache.org/dist/spark/spark-3.5.4/spark-3.5.4-bin-hadoop3.tgz && \
    tar -xf spark-3.5.4-bin-hadoop3.tgz -C /opt && \
    mv /opt/spark-3.5.4-bin-hadoop3 /opt/spark && \
    rm spark-3.5.4-bin-hadoop3.tgz

ENV SPARK_HOME=/opt/spark
ENV PATH=$SPARK_HOME/bin:$PATH
ENV PYTHONPATH=$SPARK_HOME/python:$SPARK_HOME/python/lib/py4j-0.10.9.7-src.zip:$PYTHONPATH

# Add AWS Hadoop dependencies for Spark S3 connectivity
RUN curl -O https://repo1.maven.org/maven2/org/apache/hadoop/hadoop-aws/3.3.4/hadoop-aws-3.3.4.jar && \
    curl -O https://repo1.maven.org/maven2/com/amazonaws/aws-java-sdk-bundle/1.12.261/aws-java-sdk-bundle-1.12.261.jar && \
    mv hadoop-aws-3.3.4.jar aws-java-sdk-bundle-1.12.261.jar $SPARK_HOME/jars/

USER airflow

# Install Python dependencies
COPY airflow/requirements.txt /opt/airflow/requirements.txt
RUN pip install --no-cache-dir -r /opt/airflow/requirements.txt

# Copy utils package
COPY src/utils /opt/airflow/utils/

# Copy GitHub collector package
COPY src/ /opt/airflow/src/

# Copy environment file
COPY .env /opt/airflow/.env

# Set the AIRFLOW_HOME environment variable
ENV AIRFLOW_HOME=/opt/airflow

# Initialize the database and create admin user
COPY airflow/config/airflow.cfg ${AIRFLOW_HOME}/airflow.cfg

EXPOSE 8080

# The entrypoint is already set in the base image
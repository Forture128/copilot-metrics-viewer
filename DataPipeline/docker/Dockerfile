# Use the official Python image as the base image
FROM python:3.12.6-slim

# Set the working directory in the container
WORKDIR /app

# Install Java for PySpark
RUN apt-get update && apt-get install -y openjdk-11-jdk && \
    apt-get clean

# Set environment variables for Java and Spark
ENV JAVA_HOME=/usr/lib/jvm/java-11-openjdk-amd64
ENV PATH=$JAVA_HOME/bin:$PATH

# Install Spark dependencies
# Install curl with apt-get and clean up the apt cache to reduce image size
RUN apt-get install -y curl && \
    rm -rf /var/lib/apt/lists/* && \
    # Download Apache Spark 3.2.1 binary with Hadoop 3.2 from the Apache archive
    ADD https://archive.apache.org/dist/spark/spark-3.2.1/spark-3.2.1-bin-hadoop3.2.tgz /tmp/spark-3.2.1-bin-hadoop3.2.tgz && \
    # Extract the downloaded tarball to /opt directory
    tar -xvf /tmp/spark-3.2.1-bin-hadoop3.2.tgz -C /opt && \
    # Rename the extracted directory to a simpler name
    mv /opt/spark-3.2.1-bin-hadoop3.2 /opt/spark && \
    # Remove the downloaded tarball to save space
    rm /tmp/spark-3.2.1-bin-hadoop3.2.tgz && \
    # Clean up the apt cache again to ensure the image is as small as possible
    rm -rf /var/lib/apt/lists/*

ENV SPARK_HOME=/opt/spark
ENV PATH=$SPARK_HOME/bin:$PATH

# Install dependencies from Pipfile via Pipenv
COPY Pipfile Pipfile.lock ./
RUN pip install pipenv && pipenv install --deploy --system

# Expose Jupyter's default port
EXPOSE 8888

# Command to start Jupyter Notebook
CMD ["jupyter", "notebook", "--ip=0.0.0.0", "--allow-root", "--no-browser"]
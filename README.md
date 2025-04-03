# Copilot Metrics Viewer

## Introduction

The Copilot Metrics Viewer is a comprehensive project designed to collect, process, and visualize metrics related to GitHub Copilot usage. This project is divided into two main components:

1. **Front-End (FE)**: A Vue.js application that provides a user interface for visualizing the collected metrics.
2. **DataPipeline**: A data pipeline that handles the collection, processing, and storage of metrics data.

## Project Structure

```bash
COPILOT-METRICS-VIEWER/
├── DataPipeline/ # Data Pipeline project
├── FE/ # Front-End project
├── BE/ # Back-End project (future)
└── README.md

```

## Components

### Front-End (FE)

The front-end component is a Vue.js application that visualizes the metrics collected by the data pipeline. It provides various charts and graphs to help users understand their Copilot usage.

For more details, refer to the [FE README](FE/README.md).

### DataPipeline

The data pipeline component is responsible for collecting, processing, and storing metrics data. It uses various tools and technologies such as Kafka, PySpark, and Airflow to handle the data.

For more details, refer to the [DataPipeline README](DataPipeline/README.md).

### Back-End (BE)

The Horus-BE is a backend service that provides an API for the front-end to fetch data.

For more details, refer to the [Horus-BE README](Horus-BE/README.md).

## Getting Started

To get started with the Copilot Metrics Viewer, follow the instructions in the respective README files for the Front-End and DataPipeline components.

- [FE README](FE/README.md)
- [DataPipeline README](DataPipeline/README.md)

This [README.md](http://_vscodecontentref_/#%7B%22uri%22%3A%7B%22%24mid%22%3A1%2C%22fsPath%22%3A%22%2FUsers%2Fnguyen.truong.an%2FPersonals%2Fcopilot-metrics-viewer%2FREADME.md%22%2C%22path%22%3A%22%2FUsers%2Fnguyen.truong.an%2FPersonals%2Fcopilot-metrics-viewer%2FREADME.md%22%2C%22scheme%22%3A%22file%22%7D%7D) provides an introduction to the project and includes navigation links to the [FE](http://_vscodecontentref_/#%7B%22uri%22%3A%7B%22%24mid%22%3A1%2C%22fsPath%22%3A%22%2FUsers%2Fnguyen.truong.an%2FPersonals%2Fcopilot-metrics-viewer%2FFE%22%2C%22path%22%3A%22%2FUsers%2Fnguyen.truong.an%2FPersonals%2Fcopilot-metrics-viewer%2FFE%22%2C%22scheme%22%3A%22file%22%7D%7D) and [DataPipeline](http://_vscodecontentref_/#%7B%22uri%22%3A%7B%22%24mid%22%3A1%2C%22fsPath%22%3A%22%2FUsers%2Fnguyen.truong.an%2FPersonals%2Fcopilot-metrics-viewer%2FDataPipeline%22%2C%22path%22%3A%22%2FUsers%2Fnguyen.truong.an%2FPersonals%2Fcopilot-metrics-viewer%2FDataPipeline%22%2C%22scheme%22%3A%22file%22%7D%7D) README files for more detailed information.

## Security Guidelines

### Environment Configuration

This project uses environment variables for configuration. To ensure security:

1. **Never commit sensitive information**

   - All `.env` files are ignored by git
   - Use `.env.example` and `.env.test.example` as templates
   - Keep your actual `.env` files secure and local

2. **Required Environment Files**

   - Frontend: `FE/.env` (template: `FE/.env.example`)
   - Backend: `Horus-BE/.env.test` (template: `Horus-BE/.env.test.example`)

3. **GitHub Token Security**

   - Create tokens with minimal required scopes:
     - `copilot`
     - `manage_billing:copilot` or `manage_billing:enterprise`
     - `read:enterprise`
     - `read:org`
   - Never share or commit tokens
   - Rotate tokens regularly
   - Use environment variables for token storage

4. **Database Security**

   - Use separate databases for development, testing, and production
   - Never use production credentials in test environments
   - Keep database credentials secure
   - Use strong passwords for all database users

5. **Development Best Practices**
   - Use mocked data for development (`VUE_APP_MOCKED_DATA=True`)
   - Keep test environments isolated from production
   - Use local development servers when possible
   - Follow the principle of least privilege

### Security Checklist

Before deploying or sharing code:

- [ ] No sensitive credentials in code or configuration
- [ ] All `.env` files properly ignored
- [ ] Example files contain no real data
- [ ] GitHub tokens have minimal required scopes
- [ ] Test environments use separate databases
- [ ] No production credentials in test files
- [ ] All security-related documentation is up to date

### Reporting Security Issues

If you discover a security vulnerability:

1. Do not create public issues
2. Contact the project maintainers privately
3. Provide detailed information about the vulnerability
4. Allow time for assessment and response

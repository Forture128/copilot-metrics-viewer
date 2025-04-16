# Horus - Organizational Data Analytics Platform

## Introduction

Horus is a comprehensive solution designed to collect, process, and analyze organizational data from platforms such as **GitHub**, **Jira**, and **Slack**, **utilizing APIs and webhooks** for data extraction to ensure technical transparency and data accuracy.

Data will be collected on a scheduled basis, with options for both real-time ingestion and periodic batch processing. This project aims to provide actionable insights into various productivity and delivery metrics to enhance data-driven decision-making across the organization.

## Project Structure

```bash
HORUS/
├── DataPipeline/ # Data collection and processing pipeline
├── FE/ # Front-End visualization dashboard
├── Horus-BE/ # Back-End API services
└── README.md
```

## Components

### Front-End (FE)

The front-end component is a modern web application that provides interactive dashboards and visualizations for organizational metrics. It offers:

- Real-time data visualization
- Customizable dashboards
- Cross-platform data correlation
- Export capabilities for reports

For more details, refer to the [FE README](FE/README.md).

### DataPipeline

The data pipeline component is responsible for collecting, processing, and storing data from multiple sources. It features:

- Multi-source data ingestion (GitHub, Jira, Slack)
- Real-time and batch processing capabilities
- Data transformation and enrichment
- Automated scheduling and monitoring

For more details, refer to the [DataPipeline README](DataPipeline/README.md).

### Back-End (Horus-BE)

The back-end component serves as a critical security and access control layer, providing:

- **Role-Based Access Control (RBAC)**

  - User role management and permissions
  - Organization-level access control
  - Team-based permission inheritance
  - Custom role creation and assignment

- **Organization Structure Management**

  - Hierarchical organization modeling
  - Team and department management
  - User-organization relationships
  - Cross-organization collaboration rules

- **API Gateway & Proxy Services**

  - Secure proxy to GitHub API
  - Rate limiting and request management
  - Token management and rotation
  - API request validation and sanitization

- **Security & Authentication**
  - OAuth2 and JWT-based authentication
  - Multi-factor authentication support
  - Session management
  - Audit logging and monitoring

For more details, refer to the [Horus-BE README](Horus-BE/README.md).

## Getting Started

To get started with Horus, follow the instructions in the respective README files for each component:

- [FE README](FE/README.md)
- [DataPipeline README](DataPipeline/README.md)
- [Horus-BE README](Horus-BE/README.md)

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

3. **API Token Security**

   - Create tokens with minimal required scopes
   - Never share or commit tokens
   - Rotate tokens regularly
   - Use environment variables for token storage

4. **Database Security**

   - Use separate databases for development, testing, and production
   - Never use production credentials in test environments
   - Keep database credentials secure
   - Use strong passwords for all database users

5. **Development Best Practices**
   - Use mocked data for development
   - Keep test environments isolated from production
   - Use local development servers when possible
   - Follow the principle of least privilege

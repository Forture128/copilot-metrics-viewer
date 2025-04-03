# Horus Backend Service

A Rust-based backend service focused on Role-Based Access Control (RBAC) management and organization structure.

[![Rust](https://img.shields.io/badge/rust-1.75.0+-blue.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)

## Table of Contents

- [Overview](#overview)
- [Current Focus](#current-focus)
- [Technical Stack](#technical-stack)
- [Quick Start](#quick-start)
- [Development Guide](#development-guide)
- [API Documentation](#api-documentation)
- [Testing](#testing)
- [Deployment](#deployment)
- [Architecture](#architecture)
- [Contributing](#contributing)
- [License](#license)
- [Contact](#contact)
- [Legacy Information](#legacy-information)

## Overview

Horus is a backend service that primarily manages Role-Based Access Control (RBAC) and organization structure.

## Current Focus

### RBAC Management

- **Organization Management**: Create and manage organizations
- **User Management**: User registration, authentication, and authorization
- **Team Management**: Department and team structure
- **Role Management**: Define and assign roles
- **Permission Management**: Fine-grained access control
- **Multi-tenant Support**: Organization-level isolation

### Key Features

- **Hierarchical Organization Structure**: Support for departments and teams
- **Flexible Role System**: Customizable roles and permissions
- **Secure Authentication**: JWT-based authentication
- **Audit Logging**: Track user actions and changes
- **API Integration**: RESTful API for external systems

## Technical Stack

### Core Technologies

- **Language**: Rust (2021 Edition)
- **Web Framework**: Actix Web 4.9.0
- **Database**: PostgreSQL 13+ with Diesel ORM
- **Caching**: Redis 6+
- **Authentication**: JWT with Argon2 password hashing
- **API Documentation**: utoipa with Swagger UI
- **Event Processing**: Kafka (optional)

### Key Dependencies

- `actix-web`: Web framework
- `diesel`: Database ORM
- `octocrab`: GitHub API client
- `redis`: Redis client
- `tracing`: Logging and observability
- `utoipa`: API documentation
- `serde`: Serialization/deserialization
- `validator`: Input validation
- `argon2`: Password hashing
- `jsonwebtoken`: JWT implementation

## Quick Start

### Prerequisites

- Rust (latest stable)
- PostgreSQL 13+
- Redis 6+
- GitHub API access token
- Docker (optional, for development)

### Environment Setup

1. Clone the repository:

   ```bash
   git clone https://github.com/your-org/horus-be.git
   cd horus-be
   ```

2. Create a `.env` file with required environment variables:

   ```env
   DATABASE_URL=postgres://user:password@localhost/horus
   REDIS_URL=redis://localhost
   GITHUB_TOKEN=your_github_token
   JWT_SECRET=your_jwt_secret
   LOG_LEVEL=info
   ```

3. Install dependencies:

   ```bash
   cargo build
   ```

4. Run database migrations:

   ```bash
   diesel migration run
   ```

5. Start the development server:
   ```bash
   cargo run
   ```

### Using Makefile

The project includes a Makefile for common development tasks:

```bash
# Build the project
make build

# Run tests
make test

# Run database migrations
make migrate

# Start development server
make dev

# Run linter
make lint

# Format code
make fmt

# Clean build artifacts
make clean
```

## Development Guide

### Code Organization

- Follow DDD principles (see [DEEP_DIVE.md](./DEEP_DIVE.md))
- Keep domain logic in the domain layer
- Use dependency injection for external services
- Write unit tests for domain logic
- Write integration tests for API endpoints

### Database Migrations

```bash
# Generate new migration
diesel migration generate create_table_name

# Run migrations
diesel migration run

# Revert last migration
diesel migration revert

# Generate migration
diesel migration generate create_users # generate migration

# Run migration
diesel migration run # run migration

# Redo migration
diesel migration redo # redo migration

# Revert migration
diesel migration revert # revert migration

# List migration
diesel migration list # list migration

# Generate migration with name
diesel migration generate create_users --name "create_users_1" # generate migration
```

### Testing

```bash
# Run unit tests
cargo test

# Run integration tests
cargo test --test integration_tests

# Run specific test
cargo test test_name
```

## API Documentation

The API documentation is available at:

- Swagger UI: `http://localhost:8080/api/docs`
- OpenAPI Spec: `http://localhost:8080/api/docs/openapi.json`

Features:

- Interactive API testing interface
- Request/response schemas
- Authentication examples
- Error response documentation

## Deployment

### Production Build

```bash
cargo build --release
```

### Environment Variables

Required environment variables for production:

- `DATABASE_URL`
- `REDIS_URL`
- `GITHUB_TOKEN`
- `JWT_SECRET`
- `LOG_LEVEL` (default: info)

### Performance Optimization

- The release profile is optimized for production
- Link-time optimization (LTO) is enabled
- Panic is set to abort for smaller binary size
- Debug assertions are disabled

## Architecture

The project follows Domain-Driven Design (DDD) principles with a layered architecture. For a detailed explanation of the architecture, design decisions, and project structure, see [DEEP_DIVE.md](./DEEP_DIVE.md).

Key architectural components:

- Domain Layer: Core business logic and rules
- Application Layer: Use cases and coordination
- Infrastructure Layer: Technical implementation details
- Common Layer: Shared utilities and types

### Key Benefits

- **Separation of Concerns**: Each layer has a clear responsibility
- **Domain Isolation**: Business logic is independent of frameworks
- **Scalability**: Easy to add new features and domains
- **Maintainability**: Code organized around business concepts
- **Testability**: Independent testing of domain logic

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Legacy Information

For information about the legacy metrics collection and analysis features, please refer to [LEGACY_NOTE.md](./LEGACY_NOTE.md).

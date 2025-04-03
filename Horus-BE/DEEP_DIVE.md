# Horus Backend Service - Deep Dive

This document provides a detailed explanation of the Domain-Driven Design (DDD) architecture and project structure of the Horus Backend Service, which focuses on Role-Based Access Control (RBAC) management and organization structure.

## Table of Contents

- [Architecture Overview](#architecture-overview)
- [Project Structure](#project-structure)
- [Core Domains](#core-domains)
- [Legacy Domains](#legacy-domains)
- [Application Layer](#application-layer)
- [Infrastructure Layer](#infrastructure-layer)
- [Common Layer](#common-layer)
- [Design Decisions](#design-decisions)
- [Best Practices](#best-practices)
- [Future Considerations](#future-considerations)

## Architecture Overview

The project follows a layered architecture based on Domain-Driven Design principles, with a primary focus on RBAC and organization management:

```
src/
├── domain/              # Business logic and rules
├── application/         # Use cases and coordination
├── infrastructure/      # Technical implementation
└── common/             # Shared utilities
```

### Key Principles

1. **Separation of Concerns**: Each layer has a specific responsibility
2. **Dependency Rule**: Dependencies point inward
3. **Domain Isolation**: Domain logic is independent of frameworks
4. **Testability**: Each layer can be tested independently

## Project Structure

The project is organized following DDD principles with a clear separation of concerns, focusing on RBAC and organization management:

```bash
Horus-BE/
├── Cargo.toml
├── config.rs                     # Global configuration (env, secrets, etc.)
├── main.rs                       # Application entry point; bootstraps Actix server
├── migrations/                   # SQL migrations based on your DBML schema
├── src/
│   ├── application/              # API layer: controllers, routes, DTOs, middleware
│   │   ├── controllers/
│   │   │   ├── organization_controller.rs   # Endpoints for org operations
│   │   │   ├── user_controller.rs           # Endpoints for user management
│   │   │   ├── team_controller.rs           # Endpoints for teams
│   │   │   ├── role_controller.rs           # Endpoints for RBAC
│   │   │   └── ...                          # Other controllers
│   │   ├── routes/
│   │   │   ├── organization_routes.rs       # Organization APIs
│   │   │   ├── user_routes.rs               # User management
│   │   │   ├── team_routes.rs               # Team management
│   │   │   ├── role_routes.rs               # RBAC management
│   │   │   └── ...                          # Other routes
│   │   └── middleware.rs                    # Auth, RBAC, logging
│   ├── domain/                  # Domain layer: business logic
│   │   ├── organization/        # Organization bounded context
│   │   │   ├── entities.rs      # Organization, OrganizationConfig
│   │   │   ├── repositories.rs  # Persistence interfaces
│   │   │   ├── services.rs      # Domain services
│   │   │   ├── dto.rs           # Data Transfer Objects
│   │   │   └── mod.rs           # Module exports
│   │   ├── user/                # User domain
│   │   ├── teams/               # Teams domain
│   │   ├── roles/               # Roles domain
│   │   ├── permissions/         # Permissions domain
│   │   └── mod.rs               # Domain exports
│   ├── infrastructure/          # Infrastructure layer
│   │   ├── database/            # Database implementations
│   │   ├── auth/                # Authentication and authorization
│   │   └── redis.rs             # Redis integration
│   ├── common/                  # Shared utilities
│   │   ├── error.rs
│   │   ├── logging.rs
│   │   ├── utils.rs
│   │   └── mod.rs
│   └── openapi.rs               # API documentation
└── tests/                       # Integration tests
    └── integration_tests/
```

## Core Domains

The primary focus of the service is on RBAC and organization management:

### Organization Domain

```
domain/
└── organization/
    ├── entities.rs              # Organization, Department, Team
    ├── value_objects.rs         # OrganizationSettings, Config
    ├── services/
    │   └── organization_service.rs
    ├── events.rs                # Organization events
    └── repositories.rs          # Repository interfaces
```

### User Domain

```
domain/
└── user/
    ├── entities.rs              # User, UserProfile
    ├── value_objects.rs         # UserSettings, Preferences
    ├── services/
    │   └── user_service.rs
    ├── events.rs                # User events
    └── repositories.rs          # Repository interfaces
```

### Role Domain

```
domain/
└── roles/
    ├── entities.rs              # Role, Permission
    ├── value_objects.rs         # RoleDefinition, PermissionSet
    ├── services/
    │   └── role_service.rs
    ├── events.rs                # Role events
    └── repositories.rs          # Repository interfaces
```

## Legacy Domains

The service maintains legacy support for metrics domains, though the computation logic is now handled by a separate service:

### Developer Metrics Domain

```
domain/
└── developer_metrics/
    ├── entities.rs              # Legacy entities
    ├── value_objects.rs         # Legacy value objects
    ├── services/
    │   └── metrics_service.rs   # Integration with external service
    └── repositories.rs          # Legacy repository interfaces
```

Note: The metrics domains are maintained for backward compatibility, with the actual computation and analysis handled by a separate service.

## Application Layer

The application layer orchestrates domain logic and handles use cases:

```
application/
├── controllers/         # Request handlers
├── routes/             # API route definitions
└── middleware/         # Cross-cutting concerns
```

### Key Components

1. **Controllers**: Handle HTTP requests and responses
2. **Routes**: Define API endpoints
3. **Middleware**: Handle cross-cutting concerns:
   - Authentication
   - Authorization
   - Logging
   - Error handling

## Infrastructure Layer

The infrastructure layer provides technical capabilities:

```
infrastructure/
├── database/           # Database implementations
├── github_sdk/         # GitHub API client
└── redis/             # Redis integration
```

### Key Components

1. **Database**: PostgreSQL with Diesel
2. **GitHub SDK**: GitHub API client
3. **Redis**: Caching and sessions
4. **Event Publishing**: Kafka integration

## Common Layer

The common layer provides shared utilities:

```
common/
├── error.rs           # Error handling
├── logging.rs         # Logging configuration
├── types.rs           # Shared types
└── utils.rs           # Utility functions
```

## Design Decisions

### Why DDD?

1. **Complex Domain**: Metrics collection and analysis is complex
2. **Ubiquitous Language**: Maintains common language
3. **Bounded Contexts**: Clear domain separation
4. **Maintainability**: Easier to maintain and extend

### Why Rust?

1. **Performance**: High performance for data processing
2. **Safety**: Memory and thread safety
3. **Concurrency**: Excellent async support
4. **Ecosystem**: Growing library ecosystem

### Why Actix Web?

1. **Performance**: One of the fastest web frameworks
2. **Async Support**: Excellent async/await
3. **Middleware**: Flexible middleware system
4. **Type Safety**: Strong type system

## Best Practices

### Code Organization

1. Keep domain logic pure and framework-independent
2. Use dependency injection
3. Follow single responsibility principle
4. Write comprehensive tests

### Testing Strategy

1. Unit tests for domain logic
2. Integration tests for API endpoints
3. Mock external services
4. Use feature flags

### Error Handling

1. Use custom error types
2. Implement error conversion
3. Log errors appropriately
4. Return meaningful messages

### Performance

1. Use connection pooling
2. Implement caching
3. Optimize database queries
4. Use async operations

### Security

1. Implement proper authentication
2. Use role-based access control
3. Validate all inputs
4. Sanitize outputs

## Future Considerations

1. **Scalability**

   - Implement horizontal scaling
   - Add load balancing
   - Consider microservices

2. **Monitoring**

   - Add comprehensive metrics
   - Implement distributed tracing
   - Set up alerting

3. **Extensibility**

   - Support additional data sources
   - Add more metrics types
   - Implement custom plugins

4. **Documentation**
   - Add more API documentation
   - Create architecture diagrams
   - Document deployment procedures

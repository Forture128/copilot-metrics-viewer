# Horus Legacy Documentation

This document contains legacy information about the metrics collection and analysis features of Horus, which are now handled by a separate service.

## Legacy Metrics Domains

The service previously handled the following metrics domains, which are now maintained for backward compatibility:

1. Developer Productivity Metrics

   - Code contribution analysis
   - Activity tracking
   - Individual performance metrics
   - Commit frequency and patterns
   - Code review participation

2. Team Collaboration Quality

   - Code review metrics
   - Team interaction patterns
   - Knowledge sharing indicators
   - Pull request collaboration
   - Team communication analysis

3. Project Delivery Insights
   - DORA metrics (Deployment Frequency, Lead Time, Change Failure Rate, Time to Restore)
   - Release cycle analysis
   - Project health indicators
   - Sprint velocity tracking
   - Technical debt monitoring

## Legacy DDD Structure

The original Domain-Driven Design structure for metrics domains:

```bash
src/
├── domain/
│   ├── developer_metrics/           # Developer Productivity Domain
│   │   ├── mod.rs                   # Public exports for the domain
│   │   ├── entities.rs              # Developer, Commit
│   │   ├── value_objects.rs         # ActivityMetrics, CommitFrequency
│   │   ├── services/
│   │   │   ├── github_service.rs    # External GitHub interactions
│   │   │   ├── developer_metrics_service.rs # Core logic for productivity metrics
│   │   ├── events.rs                # DeveloperMetricGeneratedEvent
│   │   └── repositories.rs          # Interfaces for persistence
│   ├── collaboration_quality/       # Team Collaboration and Quality Domain
│   │   ├── mod.rs
│   │   ├── entities.rs
│   │   ├── aggregates.rs            # CollaborationQualityMetrics
│   │   ├── services/
│   │   │   ├── collaboration_quality_service.rs
│   │   └── repositories.rs
│   ├── delivery_insights/           # Project Delivery Domain
│   │   ├── mod.rs
│   │   ├── entities.rs
│   │   ├── aggregates.rs
│   │   ├── services/
│   │   │   ├── delivery_insights_service.rs
│   │   │   └── dora_service.rs
│   │   └── repositories.rs
│   └── mod.rs                        # Domain-level exports
```

## Legacy Features

- **Real-time Metrics Collection**: Automated data collection from GitHub
- **Customizable Dashboards**: Configurable views for different team needs
- **Event-driven Architecture**: Asynchronous processing of metrics
- **Caching Layer**: Redis-based caching for performance
- **Comprehensive Logging**: Structured logging with tracing

## Note

While these metrics domains are still supported, the actual computation and analysis of these metrics is now handled by a separate service. The current focus of Horus-BE is on RBAC management and organization structure.

For current project information, please refer to:

- [README.md](./README.md)
- [DEEP_DIVE.md](./DEEP_DIVE.md)

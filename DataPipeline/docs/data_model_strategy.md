# 📐 Data Modeling Strategy for the Metrics Platform

This document defines the modeling principles, storage separation, and schema governance for both TimescaleDB (real-time) and Redshift/BigQuery (warehouse) used in our data platform.

---

## 1. 🎯 Objectives

- Maintain consistent schema across systems
- Enable scalable time-series data ingestion and analytics
- Minimize risk during schema evolution
- Support both short-term operational metrics and long-term analytics

---

## 2. 🧱 Storage Layer Responsibilities

| Layer         | Purpose                        | Technology  | Granularity     | Retention    |
| ------------- | ------------------------------ | ----------- | --------------- | ------------ |
| `TimescaleDB` | Real-time metrics ingestion    | TimescaleDB | Per event       | 30–90 days   |
| `Warehouse`   | Aggregated long-term analytics | Redshift/BQ | Daily/monthly   | 12–36 months |
| `Data Lake`   | Raw archive (optional)         | S3/GCS      | Raw JSON/events | Infinite     |

---

## 3. 🧠 Modeling Guidelines

### 3.1 Fact vs Dimension

| Element  | Fact Table                                         | Dimension Table                      |
| -------- | -------------------------------------------------- | ------------------------------------ |
| Purpose  | Store numeric events                               | Add semantic meaning                 |
| Keys     | time + metric + dimension_id                       | ID + metadata (names, tags)          |
| Examples | `developer_metrics`, `fct_team_productivity_daily` | `dim_developers`, `dim_repositories` |

### 3.2 Table Naming Convention

| Layer            | Prefix | Example                       |
| ---------------- | ------ | ----------------------------- |
| Raw Ingest       | `raw_` | `raw_github_events`           |
| Staging/Cleansed | `stg_` | `stg_developer_metrics`       |
| Final Facts      | `fct_` | `fct_team_productivity_daily` |
| Dimensions       | `dim_` | `dim_repositories`            |

---

## 4. 🕐 Schema Versioning Strategy

- All changes go through SQL-based migrations (via `dbmate`, `sqlx`, etc.)
- Use timestamped filenames: `20240414T1830_create_metrics.sql`
- Fact tables are append-only (no UPDATE/DELETE)
- Dimensions evolve with caution (prefer additive changes)
- Git is the source of truth for schema

---

## 5. 🧮 Modeling by Layer

### 5.1 TimescaleDB (Real-Time)

- Use `hypertable(event_date)` for all time-series tables
- Index on `(event_date, dimension_id)`
- Apply retention policies:
  ```sql
  SELECT add_retention_policy('developer_metrics', INTERVAL '90 days');
  ```
- Use `jsonb` fields for raw enrichment (`raw_event_data`)
- Pre-aggregate using `continuous aggregates` for dashboarding

### 5.2 Redshift / BigQuery (Warehouse)

- Use partitioned tables on `time_period` (e.g., `DATE`)
- Avoid joins on raw data, pre-join or denormalize via staging
- `dim_` tables are replicated from config/BE service (Airflow sync or API pull)
- All historical metrics should come from batch ETL (Airflow, Spark, dbt)

---

## 6. 🔁 Sync Between Layers

| Source        | Target        | Method        | Frequency    |
| ------------- | ------------- | ------------- | ------------ |
| TimescaleDB   | Warehouse     | Airflow batch | Daily        |
| BE Config DB  | dim tables    | Snapshot/API  | Daily/Weekly |
| GitHub Events | raw ingestion | Stream/File   | Real-time    |

---

## 7. ⚠️ Anti-Patterns to Avoid

- ❌ Mutating metrics (no UPDATE/DELETE on fact tables)
- ❌ Using same schema across real-time & warehouse without translation layer
- ❌ Querying TimescaleDB directly from BI tools
- ❌ Duplicating dimension metadata inside fact tables
- ❌ Hardcoding schema without migration tracking

---

## 8. 🧪 Future Considerations

- Adopt dbt for warehouse model orchestration (post-PoC)
- Implement SCD Type 2 support for `dim_` tables if needed
- Enforce schema contracts on metric_type definitions
- Apply data quality checks before inserting into fact tables

---

## 9. 📌 Summary

- Use time-partitioned, append-only models for time-series metrics
- All schema must be version-controlled and migration-managed
- Sync config metadata from BE into separate dimension tables
- Maintain separation of concerns between TimescaleDB and the Warehouse

---

**Maintainer:** `@data-platform-team`  
**Last Updated:** 2025-04-14

## 10. 📚 Others

Work with DBmate Setups:

**Note: should place in db/ before using**

```bash
alias dbmate-tsdb="dbmate --migrations-dir ./migrations/tsdb"
alias dbmate-redshift="dbmate --migrations-dir ./migrations/redshift"
```

```bash
dbmate migrate
```

# 🏢 Data Warehouse Strategy (Redshift / BigQuery)

This document outlines the warehouse design strategy, principles, and lifecycle for modeling metrics in Redshift or BigQuery.

---

## 1. 🎯 Objectives

- Store long-term aggregated metrics with low-latency query support
- Enable BI tools to perform ad-hoc slicing and dicing
- Provide high availability and consistent schema

---

## 2. 🧱 Modeling Layers

| Layer     | Description                      | Example Table                  |
|-----------|----------------------------------|--------------------------------|
| `raw_`    | Raw ingested JSON/parquet        | `raw_github_events`           |
| `stg_`    | Cleaned and normalized layer     | `stg_developer_metrics`       |
| `fct_`    | Final metric facts (daily)       | `fct_team_productivity_daily` |
| `dim_`    | Dimensions from BE metadata      | `dim_repositories`            |

---

## 3. 🧮 Modeling Patterns

- All fact tables are partitioned by `DATE`
- Use `metric_type_id` as a surrogate metric reference
- Dimensions are flattened to reduce join costs
- Surrogate keys used optionally (warehouse-friendly)

---

## 4. ⛓️ Warehouse-Specific Features

### Redshift
- Use `DISTKEY` on high-cardinality columns (e.g., `organization_id`)
- Use `SORTKEY` on `time_period`
- Pre-compute aggregates into `materialized views` when needed

### BigQuery
- Partition on `DATE(time_period)`
- Cluster by dimensions (`developer_id`, `team_id`, etc.)
- Cost-per-query model → query design matters!

---

## 5. 🧼 Dimension Strategy

- `dim_` tables are synced from BE or Admin API
- ETL script (Airflow or dbt) will snapshot dimensions periodically
- SCD support optional (start_date, end_date, is_current)

---

## 6. 🔄 Ingestion Pipeline

- Airflow DAG pulls data from TimescaleDB → Redshift
- ETL happens at staging layer (e.g. normalize from raw JSON)
- Warehouse is only target – never used for real-time ingestion

---

## 7. 🧪 Validation

- Unit test transformation logic before load
- Track record count, null checks, duplicate PKs
- Validate schema compatibility using migration contracts

---

## 8. 📎 Summary

- Model facts and dimensions cleanly
- Partition everything by time
- Sync slowly-changing config from BE
- Let metrics flow into facts, not the other way around

---

**Maintainer:** `@data-platform-team`  
**Last Updated:** 2025-04-14

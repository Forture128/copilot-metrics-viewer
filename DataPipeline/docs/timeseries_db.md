# 📊 TimescaleDB in the Data Pipeline

This document outlines the usage, setup, and best practices for using TimescaleDB as the time-series data store in our metrics processing pipeline.

---

## 1. 🧠 Why TimescaleDB?

TimescaleDB is a PostgreSQL extension optimized for time-series workloads. In our system, it's used to store real-time event data like:

- `developer_metrics`
- `team_metrics`
- `project_metrics`
- `collaboration_metrics`

**Benefits:**

- Efficient time-based partitioning with `hypertables`
- Native SQL support with PostgreSQL compatibility
- Features like compression, continuous aggregates, and retention policies

---

## 2. 🏗️ Architecture Overview

We separate TimescaleDB from our Airflow/PostgreSQL metadata to ensure clean data domains:

| Database      | Purpose                |
| ------------- | ---------------------- |
| `postgres`    | Airflow metadata       |
| `timescaledb` | Developer metrics TSDB |

→ TimescaleDB runs in a separate container (`port 5433`) to avoid port conflicts.

---

## 3. ⚙️ Docker Compose Setup

Add this to your `docker-compose.yml`:

```yaml
timescaledb:
  image: timescale/timescaledb:2.12.1-pg13
  container_name: data-pipeline-timescaledb
  environment:
    - POSTGRES_DB=metrics
    - POSTGRES_USER=metrics
    - POSTGRES_PASSWORD=metrics
  ports:
    - "5433:5432"
  volumes:
    - timescale_data:/var/lib/postgresql/data
  healthcheck:
    test: ["CMD", "pg_isready", "-U", "metrics"]
    interval: 5s
    retries: 5
```

## 4. 🧱 Creating a Hypertable

A table is not time-series until you explicitly convert it.

```sql
CREATE TABLE developer_metrics (
  id serial PRIMARY KEY,
  organization_id int,
  repository_id int,
  developer_id int,
  metric_type_id int,
  metric_value float,
  event_date timestamp NOT NULL,
  raw_event_data jsonb,
  created_at timestamp DEFAULT now()
);

SELECT create_hypertable('developer_metrics', 'event_date');
```

🔍 Check if it worked:

```sql
SELECT * FROM timescaledb_information.hypertables;
```

## 5. ✅ Best Practices

### 5.1 Ingestion

```sql
CREATE INDEX ON developer_metrics (developer_id, event_date DESC);
```

### 5.2 Retention

```sql
SELECT add_retention_policy('developer_metrics', INTERVAL '30 days');
```

### 5.3 Continuous Aggregates

```sql
CREATE MATERIALIZED VIEW daily_pr_merged
WITH (timescaledb.continuous) AS
SELECT
  time_bucket('1 day', event_date) AS day,
  developer_id,
  SUM(metric_value) as pr_merged
FROM developer_metrics
WHERE metric_type_id = 1
GROUP BY day, developer_id;
```

## 6. 🧪 Connecting from Airflow / Scripts

Add a new connection in Airflow:

```bash
airflow connections add 'timescaledb_default' \
  --conn-uri 'postgresql+psycopg2://metrics:metrics@timescaledb:5432/metrics'
```

## 🔧 Useful Commands

```bash
docker exec -it data-pipeline-timescaledb psql -U metrics -d metrics
```

```sql
-- Check hypertables
SELECT * FROM timescaledb_information.hypertables;

-- List chunks
SELECT * FROM timescaledb_information.chunks WHERE hypertable_name = 'developer_metrics';

-- Drop table (dev only)
DROP TABLE IF EXISTS developer_metrics CASCADE;
```

## 📎 References

- https://docs.timescale.com/
- https://docs.timescale.com/timescaledb/latest/how-to-guides/hypertables/
- https://docs.timescale.com/timescaledb/latest/how-to-guides/data-retention/

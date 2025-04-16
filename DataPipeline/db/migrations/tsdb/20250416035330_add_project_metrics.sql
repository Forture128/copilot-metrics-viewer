
-- migrate:up

-- Table: developer_metrics
CREATE TABLE developer_metrics (
    id serial PRIMARY KEY,
    organization_id int NOT NULL,
    repository_id int NOT NULL,
    developer_id int NOT NULL,
    metric_type_id int NOT NULL,
    metric_value float NOT NULL,
    event_date timestamp NOT NULL,
    raw_event_data jsonb,
    created_at timestamp DEFAULT now()
);
SELECT create_hypertable('developer_metrics', 'event_date', if_not_exists => TRUE);
CREATE INDEX ON developer_metrics (organization_id, repository_id, developer_id, event_date DESC);

-- Table: team_metrics
CREATE TABLE team_metrics (
    id serial PRIMARY KEY,
    organization_id int NOT NULL,
    repository_id int NOT NULL,
    team_id int NOT NULL,
    metric_type_id int NOT NULL,
    metric_value float NOT NULL,
    event_date timestamp NOT NULL,
    created_at timestamp DEFAULT now()
);
SELECT create_hypertable('team_metrics', 'event_date', if_not_exists => TRUE);
CREATE INDEX ON team_metrics (organization_id, repository_id, team_id, event_date DESC);

-- Table: project_metrics
CREATE TABLE project_metrics (
    id serial PRIMARY KEY,
    organization_id int NOT NULL,
    repository_id int NOT NULL,
    metric_type_id int NOT NULL,
    metric_value float NOT NULL,
    event_date timestamp NOT NULL,
    created_at timestamp DEFAULT now()
);
SELECT create_hypertable('project_metrics', 'event_date', if_not_exists => TRUE);
CREATE INDEX ON project_metrics (organization_id, repository_id, event_date DESC);

-- Table: collaboration_metrics
CREATE TABLE collaboration_metrics (
    id serial PRIMARY KEY,
    organization_id int NOT NULL,
    repository_id int NOT NULL,
    developer_id int NOT NULL,
    metric_type_id int NOT NULL,
    metric_value float NOT NULL,
    event_date timestamp NOT NULL,
    raw_event_data jsonb,
    created_at timestamp DEFAULT now()
);
SELECT create_hypertable('collaboration_metrics', 'event_date', if_not_exists => TRUE);
CREATE INDEX ON collaboration_metrics (organization_id, repository_id, developer_id, event_date DESC);

-- migrate:down

DROP TABLE IF EXISTS collaboration_metrics;
DROP TABLE IF EXISTS project_metrics;
DROP TABLE IF EXISTS team_metrics;
DROP TABLE IF EXISTS developer_metrics;

-- Create schema if not exists
CREATE SCHEMA IF NOT EXISTS github_metrics;

-- Create table for Copilot usage data
CREATE TABLE IF NOT EXISTS github_metrics.copilot_usage (
    team_name VARCHAR(255) NOT NULL,
    total_hours FLOAT,
    active_users INTEGER,
    created_at TIMESTAMP DEFAULT GETDATE(),
    PRIMARY KEY (team_name)
)
DISTSTYLE KEY
DISTKEY (team_name)
SORTKEY (team_name);

-- Create table for team members data
CREATE TABLE IF NOT EXISTS github_metrics.team_members (
    team VARCHAR(255) NOT NULL,
    member_name VARCHAR(255) NOT NULL,
    member_email VARCHAR(255),
    created_at TIMESTAMP DEFAULT GETDATE(),
    PRIMARY KEY (team, member_name)
)
DISTSTYLE KEY
DISTKEY (team)
SORTKEY (team, member_name);

-- Create table for DORA metrics data
CREATE TABLE IF NOT EXISTS github_metrics.dora_metrics (
    repository VARCHAR(255) NOT NULL,
    deployment_frequency INTEGER,
    avg_lead_time_hours FLOAT,
    avg_mttr_hours FLOAT,
    created_at TIMESTAMP DEFAULT GETDATE(),
    PRIMARY KEY (repository)
)
DISTSTYLE KEY
DISTKEY (repository)
SORTKEY (repository); 

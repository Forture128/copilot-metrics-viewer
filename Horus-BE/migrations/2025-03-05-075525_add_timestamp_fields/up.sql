-- Your SQL goes here

-- Add created_at and updated_at to team_repos table
ALTER TABLE team_repos 
ADD COLUMN created_at TIMESTAMP NOT NULL DEFAULT NOW(),
ADD COLUMN updated_at TIMESTAMP NOT NULL DEFAULT NOW();

-- Add created_at and updated_at to departments_roles table
ALTER TABLE departments_roles 
ADD COLUMN created_at TIMESTAMP NOT NULL DEFAULT NOW(),
ADD COLUMN updated_at TIMESTAMP NOT NULL DEFAULT NOW();

-- Add created_at and updated_at to user_roles table
ALTER TABLE user_roles 
ADD COLUMN created_at TIMESTAMP NOT NULL DEFAULT NOW(),
ADD COLUMN updated_at TIMESTAMP NOT NULL DEFAULT NOW();

-- Add updated_at to team_members table (already has joined_at)
ALTER TABLE team_members 
ADD COLUMN created_at TIMESTAMP NOT NULL DEFAULT NOW(),
ADD COLUMN updated_at TIMESTAMP NOT NULL DEFAULT NOW();

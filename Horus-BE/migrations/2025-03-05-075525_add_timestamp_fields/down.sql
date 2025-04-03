-- This file should undo anything in `up.sql`

-- Remove created_at and updated_at from team_repos table
ALTER TABLE team_repos 
DROP COLUMN created_at,
DROP COLUMN updated_at;

-- Remove created_at and updated_at from departments_roles table
ALTER TABLE departments_roles 
DROP COLUMN created_at,
DROP COLUMN updated_at;

-- Remove created_at and updated_at from user_roles table
ALTER TABLE user_roles 
DROP COLUMN created_at,
DROP COLUMN updated_at;

-- Remove updated_at from team_members table
ALTER TABLE team_members 
DROP COLUMN updated_at;

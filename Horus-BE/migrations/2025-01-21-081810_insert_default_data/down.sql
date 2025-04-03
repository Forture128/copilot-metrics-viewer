-- This file should undo anything in `up.sql`
-- Rollback case
DELETE FROM user_roles WHERE user_id IN (SELECT id FROM users);
DELETE FROM users WHERE email = 'aiden@example.com';
DELETE FROM roles WHERE name in ('admin', 'viewer');


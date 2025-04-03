-- Insert default roles
INSERT INTO roles (name, description) VALUES
('admin', 'Full Access'),
('viewer', 'Read Only');

-- Create default user
INSERT INTO users (email, password)
VALUES ('aiden@example.com', 'aidenpassword');


-- Link the admin user to the admin role
INSERT INTO user_roles (user_id, role_id)
SELECT u.id, r.id
FROM users u, roles r
WHERE u.email = 'aiden@example.com' AND r.name = 'amdin';


-- Your SQL goes here

-- Add Super Admin organization
INSERT INTO organizations (name) 
VALUES ('System') 
ON CONFLICT DO NOTHING;

-- Get the organization ID for System organization
DO $$
DECLARE
    sys_org_id INTEGER;
BEGIN
    SELECT id INTO sys_org_id FROM organizations WHERE name = 'System';
    
    -- Add Super Admin role
    INSERT INTO roles (organization_id, name, description)
    VALUES (sys_org_id, 'super_admin', 'Super Administrator with full system access')
    ON CONFLICT DO NOTHING;
    
    -- Add Admin user with hashed password (default: adminpass123)
    -- In production, you should use a secure password and proper hashing
    INSERT INTO users (organization_id, username, email, password)
    VALUES (
        sys_org_id, 
        'admin', 
        'admin@system.local', 
        -- This is a placeholder password hash - in production use proper hashing
        '$2a$12$K3JNi5nYhJH8Qb96Brj5muS0A3YlOdPXuBarRvidPAcW9UMRJlz0K'
    )
    ON CONFLICT DO NOTHING;
    
    -- Assign super_admin role to admin user
    INSERT INTO user_roles (organization_id, user_id, role_id)
    SELECT sys_org_id, u.id, r.id
    FROM users u, roles r
    WHERE u.email = 'admin@system.local' 
    AND r.name = 'super_admin'
    AND u.organization_id = sys_org_id
    AND r.organization_id = sys_org_id
    ON CONFLICT DO NOTHING;
END $$;

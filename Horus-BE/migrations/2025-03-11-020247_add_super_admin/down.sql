-- This file should undo anything in `up.sql`

-- Find the System organization ID
DO $$
DECLARE
    sys_org_id INTEGER;
BEGIN
    -- Get System organization ID
    SELECT id INTO sys_org_id FROM organizations WHERE name = 'System';
    
    -- Delete super_admin user_roles
    DELETE FROM user_roles 
    WHERE organization_id = sys_org_id 
    AND role_id IN (SELECT id FROM roles WHERE name = 'super_admin' AND organization_id = sys_org_id);
    
    -- Delete admin user
    DELETE FROM users 
    WHERE email = 'admin@system.local' 
    AND organization_id = sys_org_id;
    
    -- Delete super_admin role
    DELETE FROM roles 
    WHERE name = 'super_admin' 
    AND organization_id = sys_org_id;
    
    -- Delete System organization if no other dependencies
    DELETE FROM organizations 
    WHERE name = 'System' 
    AND id = sys_org_id;
END $$;

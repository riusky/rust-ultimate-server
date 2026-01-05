-- ============================================================================
-- DEV SEED DATA
-- ============================================================================

-- region: --- Users

-- root user (at id = 0)
INSERT INTO "user" 
    (id,  typ, username, cid, ctime, mid, mtime) VALUES 
    (0, 'Sys', 'root',  0,   now(), 0,   now());

-- User demo1 (admin user)
INSERT INTO "user" 
    (username, cid, ctime, mid, mtime) VALUES 
    ('demo1',  0,   now(), 0,   now());

-- User demo2 (regular user)
INSERT INTO "user" 
    (username, cid, ctime, mid, mtime) VALUES 
    ('demo2',  0,   now(), 0,   now());

-- Test users (60 additional users for testing)
INSERT INTO "user" 
    (username, cid, ctime, mid, mtime) VALUES 
    ('test_user_01',  0,   now(), 0,   now()),
    ('test_user_02',  0,   now(), 0,   now()),
    ('test_user_03',  0,   now(), 0,   now()),
    ('test_user_04',  0,   now(), 0,   now()),
    ('test_user_05',  0,   now(), 0,   now()),
    ('test_user_06',  0,   now(), 0,   now()),
    ('test_user_07',  0,   now(), 0,   now()),
    ('test_user_08',  0,   now(), 0,   now()),
    ('test_user_09',  0,   now(), 0,   now()),
    ('test_user_10',  0,   now(), 0,   now()),
    ('test_user_11',  0,   now(), 0,   now()),
    ('test_user_12',  0,   now(), 0,   now()),
    ('test_user_13',  0,   now(), 0,   now()),
    ('test_user_14',  0,   now(), 0,   now()),
    ('test_user_15',  0,   now(), 0,   now()),
    ('test_user_16',  0,   now(), 0,   now()),
    ('test_user_17',  0,   now(), 0,   now()),
    ('test_user_18',  0,   now(), 0,   now()),
    ('test_user_19',  0,   now(), 0,   now()),
    ('test_user_20',  0,   now(), 0,   now()),
    ('test_user_21',  0,   now(), 0,   now()),
    ('test_user_22',  0,   now(), 0,   now()),
    ('test_user_23',  0,   now(), 0,   now()),
    ('test_user_24',  0,   now(), 0,   now()),
    ('test_user_25',  0,   now(), 0,   now()),
    ('test_user_26',  0,   now(), 0,   now()),
    ('test_user_27',  0,   now(), 0,   now()),
    ('test_user_28',  0,   now(), 0,   now()),
    ('test_user_29',  0,   now(), 0,   now()),
    ('test_user_30',  0,   now(), 0,   now()),
    ('test_user_31',  0,   now(), 0,   now()),
    ('test_user_32',  0,   now(), 0,   now()),
    ('test_user_33',  0,   now(), 0,   now()),
    ('test_user_34',  0,   now(), 0,   now()),
    ('test_user_35',  0,   now(), 0,   now()),
    ('test_user_36',  0,   now(), 0,   now()),
    ('test_user_37',  0,   now(), 0,   now()),
    ('test_user_38',  0,   now(), 0,   now()),
    ('test_user_39',  0,   now(), 0,   now()),
    ('test_user_40',  0,   now(), 0,   now()),
    ('test_user_41',  0,   now(), 0,   now()),
    ('test_user_42',  0,   now(), 0,   now()),
    ('test_user_43',  0,   now(), 0,   now()),
    ('test_user_44',  0,   now(), 0,   now()),
    ('test_user_45',  0,   now(), 0,   now()),
    ('test_user_46',  0,   now(), 0,   now()),
    ('test_user_47',  0,   now(), 0,   now()),
    ('test_user_48',  0,   now(), 0,   now()),
    ('test_user_49',  0,   now(), 0,   now()),
    ('test_user_50',  0,   now(), 0,   now()),
    ('test_user_51',  0,   now(), 0,   now()),
    ('test_user_52',  0,   now(), 0,   now()),
    ('test_user_53',  0,   now(), 0,   now()),
    ('test_user_54',  0,   now(), 0,   now()),
    ('test_user_55',  0,   now(), 0,   now()),
    ('test_user_56',  0,   now(), 0,   now()),
    ('test_user_57',  0,   now(), 0,   now()),
    ('test_user_58',  0,   now(), 0,   now()),
    ('test_user_59',  0,   now(), 0,   now()),
    ('test_user_60',  0,   now(), 0,   now());

-- endregion: --- Users

-- region: --- Agents

-- Agent mock-01 (with 'parrot' model) (id: 100)
INSERT INTO "agent"    
    (id,  owner_id, name,      cid, ctime, mid, mtime) VALUES
    (100, 0,        'mock-01', 0,   now(), 0,   now());

-- endregion: --- Agents

-- ============================================================================
-- ACS SEED DATA (Roles and Associations)
-- Note: Permissions are auto-synced from code at startup
-- ============================================================================

-- region: --- Roles

INSERT INTO role (name, display_name, description, cid, ctime, mid, mtime) VALUES
    ('admin', 'Administrator', 'Full system access', 0, now(), 0, now()),
    ('user', 'Regular User', 'Standard user access', 0, now(), 0, now()),
    ('agent_manager', 'Agent Manager', 'Can manage agents', 0, now(), 0, now()),
    ('viewer', 'Viewer', 'Read-only access', 0, now(), 0, now());

-- endregion: --- Roles

-- Note: Role-Permission associations are created AFTER permission sync in Rust code
-- The following will be created via code or manually after first startup

-- region: --- User-Role Associations

-- root user: admin role
INSERT INTO user_role (user_id, role_id, cid, ctime, mid, mtime)
SELECT u.id, r.id, 0, now(), 0, now()
FROM "user" u, role r
WHERE u.username = 'root' AND r.name = 'admin';

-- demo1 user: admin role (for testing)
INSERT INTO user_role (user_id, role_id, cid, ctime, mid, mtime)
SELECT u.id, r.id, 0, now(), 0, now()
FROM "user" u, role r
WHERE u.username = 'demo1' AND r.name = 'admin';

-- demo2 user: regular user role
INSERT INTO user_role (user_id, role_id, cid, ctime, mid, mtime)
SELECT u.id, r.id, 0, now(), 0, now()
FROM "user" u, role r
WHERE u.username = 'demo2' AND r.name = 'user';

-- endregion: --- User-Role Associations

-- region: --- Role-Permission Associations

-- Give 'user' role basic permissions (for demo2)
INSERT INTO role_permission (role_id, permission_id, cid, ctime, mid, mtime)
SELECT r.id, p.id, 0, now(), 0, now()
FROM role r, permission p
WHERE r.name = 'user' AND p.key IN (
    'agent:read',
    'agent:list',
    'conv:read',
    'conv:list',
    'conv:create',
    'conv_msg:read',
    'conv_msg:create'
)
ON CONFLICT DO NOTHING;
-- endregion: --- Role-Permission Associations

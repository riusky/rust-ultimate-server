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

-- endregion: --- Users

-- region: --- Agents

-- Agent mock-01 (with 'parrot' model) (id: 100)
INSERT INTO "agent"    
    (id,  owner_id, name,      cid, ctime, mid, mtime) VALUES
    (100, 0,        'mock-01', 0,   now(), 0,   now());

-- endregion: --- Agents

-- ============================================================================
-- ACS SEED DATA (Permissions, Roles, Associations)
-- ============================================================================

-- region: --- Permissions

-- Agent CRUD permissions
INSERT INTO permission (key, group_name, display_name, description, cid, ctime, mid, mtime) VALUES
    ('agent:create', 'Agent', 'Create Agent', 'Permission to create new agents', 0, now(), 0, now()),
    ('agent:read', 'Agent', 'View Agent', 'Permission to view agent details', 0, now(), 0, now()),
    ('agent:update', 'Agent', 'Update Agent', 'Permission to update agents', 0, now(), 0, now()),
    ('agent:delete', 'Agent', 'Delete Agent', 'Permission to delete agents', 0, now(), 0, now()),
    ('agent:list', 'Agent', 'List Agents', 'Permission to list agents', 0, now(), 0, now());

-- Agent custom permissions
INSERT INTO permission (key, group_name, display_name, description, cid, ctime, mid, mtime) VALUES
    ('agent_custom:create', 'Agent Custom', 'Clone Agent', 'Permission to clone agents', 0, now(), 0, now()),
    ('agent_custom:read', 'Agent Custom', 'View Agent Stats', 'Permission to view agent statistics', 0, now(), 0, now()),
    ('agent_custom:delete', 'Agent Custom', 'Batch Delete Agents', 'Permission to batch delete agents', 0, now(), 0, now());

-- Conv CRUD permissions
INSERT INTO permission (key, group_name, display_name, description, cid, ctime, mid, mtime) VALUES
    ('conv:create', 'Conversation', 'Create Conversation', 'Permission to create conversations', 0, now(), 0, now()),
    ('conv:read', 'Conversation', 'View Conversation', 'Permission to view conversation details', 0, now(), 0, now()),
    ('conv:update', 'Conversation', 'Update Conversation', 'Permission to update conversations', 0, now(), 0, now()),
    ('conv:delete', 'Conversation', 'Delete Conversation', 'Permission to delete conversations', 0, now(), 0, now()),
    ('conv:list', 'Conversation', 'List Conversations', 'Permission to list conversations', 0, now(), 0, now());

-- Conv message permissions
INSERT INTO permission (key, group_name, display_name, description, cid, ctime, mid, mtime) VALUES
    ('conv_msg:create', 'Conversation Message', 'Add Message', 'Permission to add messages to conversations', 0, now(), 0, now()),
    ('conv_msg:read', 'Conversation Message', 'View Messages', 'Permission to view conversation messages', 0, now(), 0, now());

-- User management permissions
INSERT INTO permission (key, group_name, display_name, description, cid, ctime, mid, mtime) VALUES
    ('user:create', 'User', 'Create User', 'Permission to create users', 0, now(), 0, now()),
    ('user:read', 'User', 'View User', 'Permission to view user details', 0, now(), 0, now()),
    ('user:update', 'User', 'Update User', 'Permission to update users', 0, now(), 0, now()),
    ('user:delete', 'User', 'Delete User', 'Permission to delete users', 0, now(), 0, now()),
    ('user:list', 'User', 'List Users', 'Permission to list users', 0, now(), 0, now());

-- Role management permissions
INSERT INTO permission (key, group_name, display_name, description, cid, ctime, mid, mtime) VALUES
    ('role:create', 'Role', 'Create Role', 'Permission to create roles', 0, now(), 0, now()),
    ('role:read', 'Role', 'View Role', 'Permission to view role details', 0, now(), 0, now()),
    ('role:update', 'Role', 'Update Role', 'Permission to update roles', 0, now(), 0, now()),
    ('role:delete', 'Role', 'Delete Role', 'Permission to delete roles', 0, now(), 0, now()),
    ('role:list', 'Role', 'List Roles', 'Permission to list roles', 0, now(), 0, now());

-- Permission management permissions
INSERT INTO permission (key, group_name, display_name, description, cid, ctime, mid, mtime) VALUES
    ('permission:read', 'Permission', 'View Permission', 'Permission to view permissions', 0, now(), 0, now()),
    ('permission:list', 'Permission', 'List Permissions', 'Permission to list permissions', 0, now(), 0, now());

-- endregion: --- Permissions

-- region: --- Roles

INSERT INTO role (name, display_name, description, cid, ctime, mid, mtime) VALUES
    ('admin', 'Administrator', 'Full system access', 0, now(), 0, now()),
    ('user', 'Regular User', 'Standard user access', 0, now(), 0, now()),
    ('agent_manager', 'Agent Manager', 'Can manage agents', 0, now(), 0, now()),
    ('viewer', 'Viewer', 'Read-only access', 0, now(), 0, now());

-- endregion: --- Roles

-- region: --- Role-Permission Associations

-- Admin role: all permissions
INSERT INTO role_permission (role_id, permission_id, cid, ctime, mid, mtime)
SELECT r.id, p.id, 0, now(), 0, now()
FROM role r, permission p
WHERE r.name = 'admin';

-- Regular user role: basic agent and conv permissions
INSERT INTO role_permission (role_id, permission_id, cid, ctime, mid, mtime)
SELECT r.id, p.id, 0, now(), 0, now()
FROM role r, permission p
WHERE r.name = 'user' 
  AND p.key IN (
    'agent:read', 'agent:list',
    'conv:create', 'conv:read', 'conv:update', 'conv:delete', 'conv:list',
    'conv_msg:create', 'conv_msg:read'
  );

-- Agent manager role: all agent permissions
INSERT INTO role_permission (role_id, permission_id, cid, ctime, mid, mtime)
SELECT r.id, p.id, 0, now(), 0, now()
FROM role r, permission p
WHERE r.name = 'agent_manager' 
  AND (p.key LIKE 'agent:%' OR p.key LIKE 'agent_custom:%');

-- Viewer role: read-only permissions
INSERT INTO role_permission (role_id, permission_id, cid, ctime, mid, mtime)
SELECT r.id, p.id, 0, now(), 0, now()
FROM role r, permission p
WHERE r.name = 'viewer' 
  AND (p.key LIKE '%:read' OR p.key LIKE '%:list');

-- endregion: --- Role-Permission Associations

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

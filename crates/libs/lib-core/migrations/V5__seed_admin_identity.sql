-- ============================================================================
-- Base server identity seed data
-- ============================================================================
--
-- Keep production/server baseline data minimal:
-- - internal root system user for system-owned records and startup tasks
-- - admin user for initial login
-- - admin role and admin assignments for root/admin
--
-- Demo agents, sample profiles, and non-admin roles must be created explicitly
-- by development fixtures or application flows, not by server migrations.

-- region: --- Users

-- Internal root system user (id = 0).
INSERT INTO "user" (id, typ, username, cid, ctime, mid, mtime)
VALUES (0, 'Sys', 'root', 0, now(), 0, now());

-- Initial administrator user. The password is initialized by
-- INIT_ADMIN_PASSWORD / service bootstrap code.
INSERT INTO "user" (username, cid, ctime, mid, mtime)
VALUES ('admin', 0, now(), 0, now());

-- endregion: --- Users

-- region: --- Roles

INSERT INTO role (name, display_name, description, cid, ctime, mid, mtime)
VALUES ('admin', 'Administrator', 'Full system access', 0, now(), 0, now());

-- endregion: --- Roles

-- region: --- User-Role Associations

-- root user: admin role
INSERT INTO user_role (user_id, role_id, cid, ctime, mid, mtime)
SELECT u.id, r.id, 0, now(), 0, now()
FROM "user" u, role r
WHERE u.username = 'root'
  AND r.name = 'admin';

-- admin user: admin role
INSERT INTO user_role (user_id, role_id, cid, ctime, mid, mtime)
SELECT u.id, r.id, 0, now(), 0, now()
FROM "user" u, role r
WHERE u.username = 'admin'
  AND r.name = 'admin';

-- endregion: --- User-Role Associations

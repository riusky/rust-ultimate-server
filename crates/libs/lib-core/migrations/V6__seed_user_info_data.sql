-- User Info permissions and seed data

-- ----------------------------
-- Records of permission for user_info
-- ----------------------------
INSERT INTO "public"."permission" VALUES (1020, 'user_info:create', 'User Management', 'Create User Info', 'Create a new user profile information', 0, now(), 0, now());
INSERT INTO "public"."permission" VALUES (1021, 'user_info:read', 'User Management', 'Read User Info', 'View user profile information', 0, now(), 0, now());
INSERT INTO "public"."permission" VALUES (1022, 'user_info:update', 'User Management', 'Update User Info', 'Modify existing user profile information', 0, now(), 0, now());
INSERT INTO "public"."permission" VALUES (1023, 'user_info:delete', 'User Management', 'Delete User Info', 'Remove user profile information', 0, now(), 0, now());
INSERT INTO "public"."permission" VALUES (1024, 'user_info:list', 'User Management', 'List User Info', 'List all user profile information', 0, now(), 0, now());

-- ----------------------------
-- Reset sequence for permission
-- ----------------------------
SELECT setval('"public"."permission_id_seq"', 1024, true);

-- ----------------------------
-- Seed data for admin user_info
-- ----------------------------
INSERT INTO "public"."user_info" (user_id, nickname, avatar, bio, email, email_verified, phone, phone_verified, gender, birthday, country, province, city, timezone, locale, theme, status, login_count, cid, ctime, mid, mtime)
VALUES (
  1000,
  'Admin',
  NULL,
  'Default administrator account',
  'admin@example.com',
  false,
  NULL,
  false,
  'Unknown',
  NULL,
  'China',
  NULL,
  NULL,
  'Asia/Shanghai',
  'zh-CN',
  'system',
  'Active',
  0,
  0, now(), 0, now()
);

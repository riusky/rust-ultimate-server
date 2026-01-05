-- Base permission seed data
-- Note: Permissions are also auto-synced from code at startup via PermissionBmc::sync_from_registry

-- Clear existing permission data (for fresh install)
TRUNCATE TABLE "public"."permission" CASCADE;

-- Records of permission
INSERT INTO "public"."permission" VALUES (1000, 'conv:create', 'Conversation Management', 'Create Conversation', 'Create a new conversation entity for chat sessions', 0, '2025-12-27 10:01:46.755048+00', 0, '2025-12-27 10:01:46.755048+00');
INSERT INTO "public"."permission" VALUES (1001, 'agent:export', 'Agent Management', 'Export Agent', 'Export agent data in various formats', 0, '2025-12-27 10:01:46.787582+00', 0, '2025-12-27 10:01:46.787582+00');
INSERT INTO "public"."permission" VALUES (1002, 'agent_custom:update', 'Agent Management', 'Update Agent Custom', 'Modify existing custom agent operations', 0, '2025-12-27 10:01:46.791435+00', 0, '2025-12-27 10:01:46.791435+00');
INSERT INTO "public"."permission" VALUES (1003, 'conv:delete', 'Conversation Management', 'Delete Conversation', 'Remove conversation entity for chat sessions', 0, '2025-12-27 10:01:46.795343+00', 0, '2025-12-27 10:01:46.795343+00');
INSERT INTO "public"."permission" VALUES (1004, 'agent_custom:create', 'Agent Management', 'Create Agent Custom', 'Create a new custom agent operations', 0, '2025-12-27 10:01:46.799279+00', 0, '2025-12-27 10:01:46.799279+00');
INSERT INTO "public"."permission" VALUES (1005, 'agent:list', 'Agent Management', 'List Agent', 'List all agent entity for AI assistants', 0, '2025-12-27 10:01:46.803443+00', 0, '2025-12-27 10:01:46.803443+00');
INSERT INTO "public"."permission" VALUES (1006, 'agent:read', 'Agent Management', 'Read Agent', 'View details of agent entity for AI assistants', 0, '2025-12-27 10:01:46.807589+00', 0, '2025-12-27 10:01:46.807589+00');
INSERT INTO "public"."permission" VALUES (1007, 'agent:archive', 'Agent Management', 'Archive Agent', 'Archive an agent to mark it as inactive', 0, '2025-12-27 10:01:46.811767+00', 0, '2025-12-27 10:01:46.811767+00');
INSERT INTO "public"."permission" VALUES (1008, 'agent_custom:list', 'Agent Management', 'List Agent Custom', 'List all custom agent operations', 0, '2025-12-27 10:01:46.814797+00', 0, '2025-12-27 10:01:46.814797+00');
INSERT INTO "public"."permission" VALUES (1009, 'agent:delete', 'Agent Management', 'Delete Agent', 'Remove agent entity for AI assistants', 0, '2025-12-27 10:01:46.819046+00', 0, '2025-12-27 10:01:46.819046+00');
INSERT INTO "public"."permission" VALUES (1010, 'agent_custom:read', 'Agent Management', 'Read Agent Custom', 'View details of custom agent operations', 0, '2025-12-27 10:01:46.822081+00', 0, '2025-12-27 10:01:46.822081+00');
INSERT INTO "public"."permission" VALUES (1011, 'conv:list', 'Conversation Management', 'List Conversation', 'List all conversation entity for chat sessions', 0, '2025-12-27 10:01:46.825579+00', 0, '2025-12-27 10:01:46.825579+00');
INSERT INTO "public"."permission" VALUES (1012, 'conv_msg:create', 'Conversation Management', 'Add Conversation Message', 'Add a new message to a conversation', 0, '2025-12-27 10:01:46.828719+00', 0, '2025-12-27 10:01:46.828719+00');
INSERT INTO "public"."permission" VALUES (1013, 'conv:update', 'Conversation Management', 'Update Conversation', 'Modify existing conversation entity for chat sessions', 0, '2025-12-27 10:01:46.833691+00', 0, '2025-12-27 10:01:46.833691+00');
INSERT INTO "public"."permission" VALUES (1014, 'agent_custom:delete', 'Agent Management', 'Delete Agent Custom', 'Remove custom agent operations', 0, '2025-12-27 10:01:46.837617+00', 0, '2025-12-27 10:01:46.837617+00');
INSERT INTO "public"."permission" VALUES (1015, 'conv:read', 'Conversation Management', 'Read Conversation', 'View details of conversation entity for chat sessions', 0, '2025-12-27 10:01:46.841123+00', 0, '2025-12-27 10:01:46.841123+00');
INSERT INTO "public"."permission" VALUES (1016, 'agent:create', 'Agent Management', 'Create Agent', 'Create a new agent entity for AI assistants', 0, '2025-12-27 10:01:46.844421+00', 0, '2025-12-27 10:01:46.844421+00');
INSERT INTO "public"."permission" VALUES (1017, 'agent:update', 'Agent Management', 'Update Agent', 'Modify existing agent entity for AI assistants', 0, '2025-12-27 10:01:46.848421+00', 0, '2025-12-27 10:01:46.848421+00');
INSERT INTO "public"."permission" VALUES (1018, 'conv_msg:read', 'Conversation Management', 'Get Conversation Message', 'View a specific message from a conversation', 0, '2025-12-27 10:01:46.85177+00', 0, '2025-12-27 10:01:46.85177+00');
INSERT INTO "public"."permission" VALUES (1019, 'user:reset-pwd', 'User Management', 'Reset User Password', 'Admin can reset any user''s password', 0, '2025-12-27 10:01:46.855511+00', 0, '2025-12-27 10:01:46.855511+00');

-- Reset sequence for permission
SELECT setval('"public"."permission_id_seq"', 1019, true);

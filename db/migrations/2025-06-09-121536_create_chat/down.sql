-- This file should undo anything in `up.sql`
DROP INDEX IF EXISTS idx_messages_created_at;
DROP INDEX IF EXISTS idx_messages_conversation_id;
DROP TABLE IF EXISTS messages;
DROP TABLE IF EXISTS conversations;
-- This file should undo anything in `up.sql`

-- Drop the triggers
DROP TRIGGER IF EXISTS on_user_relation_update ON user_relations;
DROP TRIGGER IF EXISTS on_user_relation_delete ON user_relations;

-- Drop the trigger functions
DROP FUNCTION IF EXISTS create_channel_on_acceptance() CASCADE;
DROP FUNCTION IF EXISTS delete_channel_on_relation_delete() CASCADE;

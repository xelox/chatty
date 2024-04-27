-- This file should undo anything in `up.sql`

UPDATE users SET display_name = '' WHERE display_name IS NULL;

ALTER TABLE users
ALTER COLUMN username TYPE VARCHAR,
ALTER COLUMN display_name SET DEFAULT '',
ALTER COLUMN display_name SET NOT NULL;


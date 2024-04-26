-- This file should undo anything in `up.sql`

ALTER TABLE users
ALTER COLUMN username TYPE VARCHAR,
ALTER COLUMN display_name SET NOT NULL,
ALTER COLUMN display_name SET DEFAULT '';

-- Your SQL goes here

ALTER TABLE users
ALTER COLUMN username TYPE VARCHAR(50),
ALTER COLUMN display_name DROP NOT NULL,
ALTER COLUMN display_name SET DEFAULT NULL;

UPDATE users SET display_name = NULL WHERE display_name = '';

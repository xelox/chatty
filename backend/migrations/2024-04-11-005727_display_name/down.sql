-- This file should undo anything in `up.sql`
ALTER TABLE "users"
DROP COLUMN "display_name";

ALTER TABLE "users"
ADD "auth_token" UUID;

ALTER TABLE "users"
RENAME COLUMN "unique_name" to "nick";

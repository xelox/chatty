-- Your SQL goes here
ALTER TABLE "users"
ADD "display_name" VARCHAR;

ALTER TABLE "users"
DROP COLUMN "auth_token";

ALTER TABLE "users"
RENAME COLUMN "nick" to "unique_name";

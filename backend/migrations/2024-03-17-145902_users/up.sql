-- Your SQL goes here
CREATE TABLE "users"(
	"nick" VARCHAR NOT NULL PRIMARY KEY,
	"password_hash" VARCHAR NOT NULL,
	"auth_token" UUID
);


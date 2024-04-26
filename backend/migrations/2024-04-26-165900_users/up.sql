-- Your SQL goes here

CREATE TABLE users (
  id UUID NOT NULL PRIMARY KEY,
  username VARCHAR NOT NULL UNIQUE,
  email VARCHAR(254) UNIQUE,
  display_name VARCHAR(50) NOT NULL DEFAULT '',
  password_hash CHAR(97) NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  last_online TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

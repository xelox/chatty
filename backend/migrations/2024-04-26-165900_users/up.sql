-- Your SQL goes here

CREATE TABLE users (
  id BIGINT NOT NULL PRIMARY KEY,

  username VARCHAR NOT NULL UNIQUE,
  email VARCHAR(254) UNIQUE,
  password_hash CHAR(97) NOT NULL,

  display_name VARCHAR(25) NOT NULL DEFAULT '',
  pfp_last_update TIMESTAMP,
  banner_last_update TIMESTAMP, 
  custom_status VARCHAR(50),
  about_me VARCHAR(500) NOT NULL DEFAULT '',

  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  last_online TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

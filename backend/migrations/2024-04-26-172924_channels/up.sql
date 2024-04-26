-- Your SQL goes here

CREATE TABLE channels (
  id UUID NOT NULL PRIMARY KEY,
  channel_name VARCHAR(25) NOT NULL,
  channel_description VARCHAR(255),
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  last_activity TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  subscribers_count INT NOT NULL DEFAULT 0
);

CREATE INDEX channel_last_activity_key ON channels(last_activity DESC);

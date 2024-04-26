-- Your SQL goes here

CREATE TABLE channels (
  id UUID NOT NULL PRIMARY KEY,
  channel_name VARCHAR(25) NOT NULL,
  channel_description VARCHAR(255),
  created_at BIGINT NOT NULL DEFAULT EXTRACT(epoch from CURRENT_TIMESTAMP)::BIGINT,
  last_activity BIGINT NOT NULL DEFAULT EXTRACT(epoch from CURRENT_TIMESTAMP)::BIGINT,
  subscribers_count INT NOT NULL DEFAULT 0
);

CREATE INDEX channel_last_activity_key ON channels(last_activity DESC);

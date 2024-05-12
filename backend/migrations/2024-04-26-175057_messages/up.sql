-- Your SQL goes here

CREATE DOMAIN NON_NULL_TEXT AS TEXT NOT NULL;
CREATE DOMAIN NON_NULL_BIGINT AS BIGINT NOT NULL;

CREATE TABLE messages (
  id BIGINT NOT NULL PRIMARY KEY,

  sender_id BIGINT NOT NULL,
    FOREIGN KEY (sender_id) 
      REFERENCES users(id)
      ON DELETE CASCADE,

  channel_id BIGINT NOT NULL,
    FOREIGN KEY (channel_id) 
      REFERENCES channels(id)
      ON DELETE CASCADE,

  content VARCHAR(2000) NOT NULL,
  attachments NON_NULL_TEXT[] NOT NULL DEFAULT '{}'::NON_NULL_TEXT[],
  mentions NON_NULL_BIGINT[] NOT NULL DEFAULT '{}'::NON_NULL_BIGINT[],
  reactions JSONB,

  sent_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX message_channel_key ON messages(channel_id);
CREATE INDEX message_sender_key ON messages(sender_id);
CREATE INDEX message_ordering_key ON messages(sent_at DESC);

CREATE OR REPLACE FUNCTION update_channel_last_activity()
RETURNS TRIGGER AS $$
BEGIN
  UPDATE channels
  SET last_activity = NEW.sent_at
  WHERE id = NEW.channel_id;
  RETURN NULL;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE TRIGGER after_message_insert
AFTER INSERT ON messages
FOR EACH ROW
EXECUTE FUNCTION update_channel_last_activity();

-- Your SQL goes here

CREATE TABLE messages (
  id UUID NOT NULL PRIMARY KEY,

  sender_id UUID NOT NULL,
    FOREIGN KEY (sender_id) 
      REFERENCES users(id)
      ON DELETE CASCADE,

  channel_id UUID NOT NULL,
    FOREIGN KEY (channel_id) 
      REFERENCES channels(id)
      ON DELETE CASCADE,

  content VARCHAR(2000) NOT NULL,
  attachments TEXT[] NOT NULL DEFAULT '{}'::TEXT[],
  mentions UUID[] NOT NULL DEFAULT '{}'::UUID[],
  reactions JSONB,

  sent_at BIGINT NOT NULL DEFAULT EXTRACT(epoch from CURRENT_TIMESTAMP)::BIGINT,
  updated_at BIGINT NOT NULL DEFAULT EXTRACT(epoch from CURRENT_TIMESTAMP)::BIGINT
);

CREATE INDEX message_channel_key ON messages(channel_id);
CREATE INDEX message_sender_key ON messages(sender_id);
CREATE INDEX message_ordering_key ON messages(sent_at DESC);

CREATE OR REPLACE FUNCTION update_channel_last_activity()
RETURNS TRIGGER AS $$
BEGIN
  UPDATE channels
  SET last_activity = NEW.sent_at
  WHERE channel_id = NEW.channel_id;
  RETURN NULL;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE TRIGGER after_message_insert
AFTER INSERT ON messages
FOR EACH ROW
EXECUTE FUNCTION update_channel_last_activity();

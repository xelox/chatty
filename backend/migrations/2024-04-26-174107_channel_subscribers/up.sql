-- Your SQL goes here

CREATE TABLE channel_subscribers (
  user_id UUID NOT NULL,
    FOREIGN KEY(user_id) 
      REFERENCES users(id)
      ON DELETE CASCADE,

  channel_id UUID NOT NULL,
    FOREIGN KEY(channel_id) 
      REFERENCES channels(id)
      ON DELETE CASCADE,

  PRIMARY KEY(user_id, channel_id),

  subscribed_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX channel_subscriber_user_key ON channel_subscribers(user_id);
CREATE INDEX channel_subscriber_channel_key ON channel_subscribers(channel_id);

CREATE OR REPLACE FUNCTION update_channel_subscribers_count()
RETURNS TRIGGER AS $$
BEGIN
    IF TG_OP = 'INSERT' THEN
        UPDATE channels
        SET member_count = member_count + 1
        WHERE channel_id = NEW.channel_id;
    ELSIF TG_OP = 'DELETE' THEN
        UPDATE channels
        SET member_count = member_count - 1
        WHERE channel_id = OLD.channel_id;
    END IF;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE TRIGGER after_channel_subscription_insert
AFTER INSERT ON channel_subscribers
FOR EACH ROW EXECUTE FUNCTION update_channel_subscribers_count();

CREATE OR REPLACE TRIGGER after_channel_subscription_delete
AFTER DELETE ON channel_subscribers
FOR EACH ROW EXECUTE FUNCTION update_channel_subscribers_count();

-- Your SQL goes here

-- Create trigger function to handle insertion or update of user_relations
CREATE OR REPLACE FUNCTION create_channel_on_acceptance()
RETURNS TRIGGER AS $$
DECLARE
  a_username TEXT;
  b_username TEXT;
BEGIN
  -- Check if the accepted column has been updated to true
  IF NEW.accepted = true THEN
    SELECT username INTO a_username FROM users WHERE id = NEW.a;
    SELECT username INTO b_username FROM users WHERE id = NEW.b;

    INSERT INTO channels (id, channel_name)
    VALUES (NEW.id, CONCAT(a_username, ' & ', b_username));
    
    -- Insert rows into channel_subscribers
    INSERT INTO channel_subscribers (user_id, channel_id) 
    VALUES (NEW.a, NEW.id);

    INSERT INTO channel_subscribers (user_id, channel_id) 
    VALUES (NEW.b, NEW.id);

    NEW.accepted_at := CURRENT_TIMESTAMP;
  END IF;
RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Create trigger function to handle deletion of user_relations
CREATE OR REPLACE FUNCTION delete_channel_on_relation_delete()
RETURNS TRIGGER AS $$
BEGIN
-- Delete the corresponding channel row when a user_relation row is deleted
  DELETE FROM channels WHERE id = OLD.id;
  RETURN OLD;
END;
$$ LANGUAGE plpgsql;

-- Create trigger to fire on update of user_relations
CREATE OR REPLACE TRIGGER on_user_relation_update
BEFORE UPDATE OF accepted ON user_relations
FOR EACH ROW
EXECUTE FUNCTION create_channel_on_acceptance();

-- Create trigger to fire on deletion of user_relations
CREATE OR REPLACE TRIGGER on_user_relation_delete
AFTER DELETE ON user_relations
FOR EACH ROW
EXECUTE FUNCTION delete_channel_on_relation_delete();

-- Your SQL goes here

-- Create trigger function to handle insertion or update of user_relations
CREATE OR REPLACE FUNCTION create_channel_on_acceptance()
RETURNS TRIGGER AS $$
BEGIN
    -- Check if the accepted column has been updated to true
    IF NEW.accepted = true THEN
        INSERT INTO channels (id, channel_name)
        VALUES (NEW.id, CONCAT('@', NEW.a, ' & @', NEW.b));

       -- Insert rows into channel_subscribers
        INSERT INTO channel_subscribers (channel_id, user_id) VALUES (NEW.id, NEW.a);
        INSERT INTO channel_subscribers (channel_id, user_id) VALUES (NEW.id, NEW.b);
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
AFTER UPDATE OF accepted ON user_relations
FOR EACH ROW
EXECUTE FUNCTION create_channel_on_acceptance();

-- Create trigger to fire on deletion of user_relations
CREATE OR REPLACE TRIGGER on_user_relation_delete
AFTER DELETE ON user_relations
FOR EACH ROW
EXECUTE FUNCTION delete_channel_on_relation_delete();

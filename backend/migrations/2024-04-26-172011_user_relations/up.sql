-- Your SQL goes here

CREATE TABLE user_relations (
  id BIGINT NOT NULL PRIMARY KEY,

  a BIGINT NOT NULL, 
    FOREIGN KEY (a) 
      REFERENCES users(id)
      ON DELETE CASCADE,

  b BIGINT NOT NULL, 
    FOREIGN KEY (b) 
      REFERENCES users(id)
      ON DELETE CASCADE,

  UNIQUE(a, b),

  sender BIGINT NOT NULL,
    FOREIGN KEY (sender) 
      REFERENCES users(id)
      ON DELETE CASCADE,

  accepted BOOL NOT NULL DEFAULT FALSE,

  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  accepted_at TIMESTAMP
);

CREATE INDEX friendship_sender_key on user_relations(sender);
CREATE INDEX user_relations_a_key on user_relations(a);
CREATE INDEX user_relations_b_key on user_relations(b);

-- Your SQL goes here

CREATE TABLE "friendship"(
  id UUID NOT NULL PRIMARY KEY,
  a VARCHAR NOT NULL, 
    FOREIGN KEY (a) 
      REFERENCES users(unique_name)
      ON DELETE CASCADE,

  b VARCHAR NOT NULL, 
    FOREIGN KEY (b) 
      REFERENCES users(unique_name)
      ON DELETE CASCADE,

  UNIQUE(a, b),

  sender VARCHAR NOT NULL,
  accepted BOOL NOT NULL
);

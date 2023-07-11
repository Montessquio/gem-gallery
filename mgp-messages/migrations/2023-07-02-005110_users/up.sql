-- "immutable" user data, which includes User IDs and password hashes
CREATE TABLE users (
  id UUID UNIQUE PRIMARY KEY NOT NULL,
  pw VARCHAR NOT NULL,
  salt VARCHAR NOT NULL
)

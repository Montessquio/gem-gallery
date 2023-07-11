CREATE TYPE PARENTMODE AS ENUM ('reblog', 'reply');

CREATE TABLE posts (
  id UUID UNIQUE PRIMARY KEY NOT NULL,
  body TEXT NOT NULL,
  media JSONB NOT NULL,
  author UUID NOT NULL REFERENCES users,
  published TIMESTAMP NOT NULL,
  likes BIGINT NOT NULL DEFAULT 0,
  reblogs BIGINT NOT NULL DEFAULT 0,
  comments BIGINT NOT NULL DEFAULT 0,
  mentions UUID[] NOT NULL,
  tags TEXT[] NOT NULL,
  parent UUID REFERENCES posts,
  parent_mode PARENTMODE,

  flags JSONB NOT NULL -- Moderation flags such as "shadowban" (visible only to followers), "mature content", et cetera. Not just disciplinary.
)
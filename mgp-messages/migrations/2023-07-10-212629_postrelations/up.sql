-- Your SQL goes here
-- Table describing "user liked posts, user hidden posts, etc"
CREATE TABLE postrelations (
    actor UUID NOT NULL REFERENCES users,
    target UUID NOT NULL REFERENCES posts,
    relation JSONB NOT NULL, -- info containing the type of relation, i.e. "hidden", "liked", or any further combo
    PRIMARY KEY (actor, target)
)
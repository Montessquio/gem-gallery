-- Your SQL goes here
CREATE TABLE userrelations (
    actor UUID NOT NULL REFERENCES users,
    target UUID NOT NULL REFERENCES users,
    relation JSONB NOT NULL, -- info containing the type of relation, i.e. "followed", "blocked", or any further combo
    PRIMARY KEY (actor, target)
)
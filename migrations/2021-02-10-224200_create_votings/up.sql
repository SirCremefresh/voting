-- Your SQL goes here

CREATE
    EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE votings
(
    id      VARCHAR(36) PRIMARY KEY DEFAULT uuid_generate_v4(),
    admin_key_hash VARCHAR(64) NOT NULL,
    name           VARCHAR(64) NOT NULL
);

CREATE TABLE polls
(
    id      VARCHAR(36) PRIMARY KEY DEFAULT uuid_generate_v4(),
    sequenz_number INT NOT NULL,
    voting_fk   VARCHAR(36) NOT NULL
        CONSTRAINT polls_votings_id_fk
            REFERENCES votings(id)
            ON DELETE CASCADE,
    name        VARCHAR(64) NOT NULL,
    description VARCHAR(64) NOT NULL
);

CREATE TABLE voters 
(
    id      VARCHAR(36) PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_key_hash VARCHAR(64) NOT NULL,
    voting_fk   VARCHAR(36) NOT NULL
        CONSTRAINT voters_votings_id_fk
            REFERENCES votings(id)
            ON DELETE CASCADE,
    username VARCHAR(64) NOT NULL
);

CREATE TABLE votes (
    id      VARCHAR(36) PRIMARY KEY DEFAULT uuid_generate_v4(),
    poll_fk   VARCHAR(36) NOT NULL
        CONSTRAINT votes_polls_id_fk
            REFERENCES polls(id)
            ON DELETE CASCADE,
    voter_fk   VARCHAR(36) NOT NULL
        CONSTRAINT votes_voters_id_fk
            REFERENCES voters(id)
            ON DELETE CASCADE,
    answer BOOLEAN NULL
);

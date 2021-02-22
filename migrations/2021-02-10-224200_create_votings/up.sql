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
    voting_fk   VARCHAR(36) NOT NULL
        CONSTRAINT votings_voting_id_fk
            REFERENCES votings(id)
            ON DELETE CASCADE,
    name        VARCHAR(64) NOT NULL,
    description VARCHAR(64) NOT NULL
);


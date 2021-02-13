-- Your SQL goes here

CREATE
EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE votings
(
    voting_id      VARCHAR(36) PRIMARY KEY DEFAULT uuid_generate_v4(),
    admin_key_hash VARCHAR(64) NOT NULL,
    name           VARCHAR(64)  NOT NULL
)
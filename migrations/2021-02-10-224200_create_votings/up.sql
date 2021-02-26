-- Your SQL goes here

CREATE
    EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE votings
(
    id      VARCHAR(36) PRIMARY KEY DEFAULT uuid_generate_v4(),
    admin_key_hash VARCHAR(64) NOT NULL,
    name           VARCHAR(64) NOT NULL,
    active_poll_index INT DEFAULT NULL
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
    voter_key_hash VARCHAR(64) NOT NULL,
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

CREATE VIEW poll_results as
(
select p.id,
       p.sequenz_number,
       p.voting_fk,
       p.name,
       p.description,
       count(votes_accept.id)                                                     as votes_accept,
       count(votes_decline.id)                                                    as votes_decline,
       count(votes_abstain.id)                                                    as votes_abstain,
       count(votes_accept.id) + count(votes_decline.id) + count(votes_abstain.id) as votes_total
from polls p
         left join votes votes_accept on p.id = votes_accept.poll_fk and votes_accept.answer = True
         left join votes votes_decline on p.id = votes_decline.poll_fk and votes_decline.answer = False
         left join votes votes_abstain on p.id = votes_abstain.poll_fk and votes_abstain.answer IS NULL
group by p.id, p.name, p.description, p.sequenz_number, p.voting_fk
order by p.sequenz_number
    );

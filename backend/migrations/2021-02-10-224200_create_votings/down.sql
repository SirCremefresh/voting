-- This file should undo anything in `up.sql`

DROP VIEW IF EXISTS poll_results;
DROP TABLE IF EXISTS votes;
DROP TABLE IF EXISTS voters;
DROP TABLE IF EXISTS polls;
DROP TABLE IF EXISTS votings;

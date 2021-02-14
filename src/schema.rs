table! {
    polls (poll_id) {
        poll_id -> Varchar,
        voting_fk -> Varchar,
        name -> Varchar,
        description -> Varchar,
    }
}

table! {
    votings (voting_id) {
        voting_id -> Varchar,
        admin_key_hash -> Varchar,
        name -> Varchar,
    }
}

joinable!(polls -> votings (voting_fk));

allow_tables_to_appear_in_same_query!(polls, votings,);

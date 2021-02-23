table! {
    polls (id) {
        id -> Varchar,
        sequenz_number -> Int4,
        voting_fk -> Varchar,
        name -> Varchar,
        description -> Varchar,
    }
}

table! {
    votings (id) {
        id -> Varchar,
        admin_key_hash -> Varchar,
        name -> Varchar,
    }
}

joinable!(polls -> votings (voting_fk));

allow_tables_to_appear_in_same_query!(
    polls,
    votings,
);

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
    voters (id) {
        id -> Varchar,
        user_key_hash -> Varchar,
        voting_fk -> Varchar,
        username -> Varchar,
    }
}

table! {
    votes (id) {
        id -> Varchar,
        poll_fk -> Varchar,
        voter_fk -> Varchar,
        answer -> Nullable<Bool>,
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
joinable!(voters -> votings (voting_fk));
joinable!(votes -> polls (poll_fk));
joinable!(votes -> voters (voter_fk));

allow_tables_to_appear_in_same_query!(
    polls,
    voters,
    votes,
    votings,
);

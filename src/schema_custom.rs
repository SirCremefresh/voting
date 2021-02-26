table! {
    poll_results (id) {
        id -> Varchar,
        sequenz_number -> Int4,
        voting_fk -> Varchar,
        name -> Varchar,
        description -> Varchar,
        votes_accept -> Int8,
        votes_decline -> Int8,
        votes_abstain -> Int8,
        votes_total -> Int8,
    }
}

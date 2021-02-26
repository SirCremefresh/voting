use crate::schema::polls;
use crate::schema::voters;
use crate::schema::votes;
use crate::schema::votings;
use crate::schema_custom::poll_results;

#[derive(Queryable, Serialize, Insertable, PartialEq, Identifiable, Debug)]
pub struct Voting {
    pub id: String,
    pub admin_key_hash: String,
    pub name: String,
    pub active_poll_index: Option<i32>,
}

#[derive(Queryable, Serialize, Insertable, PartialEq, Identifiable, Debug)]
pub struct Poll {
    pub id: String,
    pub sequenz_number: i32,
    pub voting_fk: String,
    pub name: String,
    pub description: String,
}

#[derive(Queryable, Serialize, Insertable, PartialEq, Identifiable, Debug)]
pub struct Voter {
    pub id: String,
    pub voter_key_hash: String,
    pub voting_fk: String,
    pub username: String,
}

#[derive(Identifiable, Queryable, PartialEq, Insertable)]
pub struct Vote {
    pub id: String,
    pub poll_fk: String,
    pub voter_fk: String,
    pub answer: Option<bool>,
}

#[derive(Queryable, PartialEq, Identifiable, Debug)]
pub struct PollResult {
    pub id: String,
    pub sequenz_number: i32,
    pub voting_fk: String,
    pub name: String,
    pub description: String,
    pub votes_accept: i64,
    pub votes_decline: i64,
    pub votes_abstain: i64,
    pub votes_total: i64,
}

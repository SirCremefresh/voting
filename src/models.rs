use crate::schema::polls;
use crate::schema::votings;
use crate::schema::voters;
use crate::schema::votes;

#[derive(Queryable, Serialize, Insertable, PartialEq, Identifiable, Debug)]
pub struct Voting {
    pub id: String,
    pub admin_key_hash: String,
    pub name: String,
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
    pub user_key_hash: String,
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
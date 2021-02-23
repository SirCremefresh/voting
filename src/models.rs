use crate::schema::polls;
use crate::schema::votings;

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

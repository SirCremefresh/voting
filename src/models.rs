use crate::schema::votings;

#[derive(Queryable, Serialize, Insertable, PartialEq, Debug)]
pub struct Voting {
    pub voting_id: String,
    pub admin_key_hash: String,
    pub name: String,
}

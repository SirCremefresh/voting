use crate::schema::votings;

#[derive(Queryable, Serialize, Insertable)]
pub struct Voting {
    pub voting_id: String,
    pub name: String,
}

use crate::schema::polls;
use crate::schema::votings;
use crate::schema::voters;
use crate::schema::votes;

use std::io::Write;

use diesel::backend::Backend;
use diesel::serialize::{self, IsNull, Output, ToSql};

#[derive(SqlType)]
#[postgres(type_name = "decision")]
pub struct DecisionType;

#[derive(Debug, FromSqlRow, AsExpression)]
#[sql_type = "DecisionType"]
pub enum Decision {
    ACCEPT,
    DECLINE,
    ABSTAIN,
}

pub mod exports {
    pub use super::Decision;
}

impl<Db: Backend> ToSql<DecisionType, Db> for Decision {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Db>) -> serialize::Result {
        match *self {
            Decision::ACCEPT => out.write_all(b"ACCEPT")?,
            Decision::DECLINE => out.write_all(b"DECLINE")?,
            Decision::ABSTAIN => out.write_all(b"ABSTAIN")?,
        }
        Ok(IsNull::No)
    }
}

use diesel::deserialize::{self, FromSql};
use diesel::pg::Pg;

impl FromSql<DecisionType, Pg> for Decision {
    fn from_sql(bytes: Option<&<Pg as Backend>::RawValue>) -> deserialize::Result<Self> {
        match not_none!(bytes) {
            b"ACCEPT" => Ok(Decision::ACCEPT),
            b"DECLINE" => Ok(Decision::DECLINE),
            b"ABSTAIN" => Ok(Decision::ABSTAIN),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

#[derive(Queryable, Insertable)]
pub struct Vote {
    pub id: String,
    pub poll_fk: String,
    pub voter_fk: String,
    pub answer: Decision,
}


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


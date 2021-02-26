use crate::pool::DbConn;

use crate::utils::ErrorResponse;

use diesel::insert_into;
use diesel::prelude::*;
use rocket::http::Status;

pub fn insert_poll(
    conn: &DbConn,
    poll_name: &String,
    poll_sequenz_number: i32,
    poll_description: &String,
    poll_voting_fk: &String,
) -> QueryResult<usize> {
    use crate::schema::polls::dsl::{description, name, polls, sequenz_number, voting_fk};

    insert_into(polls)
        .values((
            name.eq(&poll_name),
            sequenz_number.eq(poll_sequenz_number),
            description.eq(&poll_description),
            voting_fk.eq(&poll_voting_fk),
        ))
        .execute(&**conn)
}

pub fn insert_voter(
    conn: &DbConn,
    username: &String,
    voter_key_hash: &String,
    voting_id: &String,
) -> Result<(), ErrorResponse> {
    use crate::schema::voters;

    insert_into(voters::table)
        .values((
            voters::username.eq(&username),
            voters::voter_key_hash.eq(&voter_key_hash),
            voters::voting_fk.eq(&voting_id),
        ))
        .execute(&**conn)
        .map_err(|err| {
            let error_msg = format!("Could not insert voter for voting with id: {}", voting_id);
            println!("{}. err: {:?}", error_msg, err);
            ErrorResponse {
                reason: error_msg,
                status: Status::InternalServerError,
            }
        })?;
    Ok(())
}

pub fn insert_voting(
    conn: &DbConn,
    voting_name: &String,
    voting_admin_key_hash: &String,
) -> QueryResult<String> {
    use crate::schema::votings::dsl::{admin_key_hash, id, name, votings};

    insert_into(votings)
        .values((
            name.eq(&voting_name),
            admin_key_hash.eq(&voting_admin_key_hash),
        ))
        .returning(id)
        .get_result(&**conn)
}

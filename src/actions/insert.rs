use crate::pool::DbConn;

use crate::utils::ErrorResponse;

use diesel::insert_into;
use diesel::prelude::*;
use rocket::http::Status;

pub fn insert_poll(
    conn: &DbConn,
    name: &String,
    sequenz_number: i32,
    description: &String,
    voting_fk: &String,
) -> QueryResult<usize> {
    use crate::schema::polls;

    insert_into(polls::table)
        .values((
            polls::name.eq(&name),
            polls::sequenz_number.eq(sequenz_number),
            polls::description.eq(&description),
            polls::voting_fk.eq(&voting_fk),
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

<<<<<<< HEAD
pub fn insert_vote(
    conn: &DbConn,
    poll_id: &String,
    voter_id: &String,
    answer: &Option<bool>,
) -> Result<(), ErrorResponse> {
    use crate::schema::votes;

    insert_into(votes::table)
        .values((
            votes::poll_fk.eq(&poll_id),
            votes::voter_fk.eq(&voter_id),
            votes::answer.eq(answer),
        ))
        .execute(&**conn)
        .map_err(|err| {
            let error_msg = format!(
                "Could not insert vote for poll id: {} and voter id: {} with answer: {:?}",
                poll_id, voter_id, answer
            );
            println!("{}. err: {:?}", error_msg, err);
            ErrorResponse {
                reason: error_msg,
                status: Status::InternalServerError,
            }
        })?;

    Ok(())
}

=======
>>>>>>> 5ddd46a444d5850df3078e8f6e9f699fe66b83d3
pub fn insert_voting(conn: &DbConn, name: &String, admin_key_hash: &String) -> QueryResult<String> {
    use crate::schema::votings;

    insert_into(votings::table)
        .values((
            votings::name.eq(&name),
            votings::admin_key_hash.eq(&admin_key_hash),
        ))
        .returning(votings::id)
        .get_result(&**conn)
}

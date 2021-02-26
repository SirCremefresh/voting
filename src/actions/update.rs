use crate::models::*;
use crate::pool::DbConn;
use crate::utils::ErrorResponse;
use diesel::prelude::*;
use rocket::http::Status;

pub fn update_voting(
    conn: &DbConn,
    voting: &Voting,
    poll_index: &Option<i32>,
) -> Result<(), ErrorResponse> {
    use crate::schema::votings;

    diesel::update(voting)
        .set(votings::active_poll_index.eq(poll_index))
        .execute(&**conn)
        .map_err(|err| {
            let error_msg = format!(
                "Could not set active_poll_index: {:?} for voting with id: {}",
                poll_index, &voting.id
            );
            println!("{}. err: {:?}", error_msg, err);
            ErrorResponse {
                reason: error_msg,
                status: Status::InternalServerError,
            }
        })?;

    Ok(())
}

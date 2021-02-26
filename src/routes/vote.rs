use crate::pool::DbConn;

use crate::actions::check::*;
use crate::actions::find::*;
<<<<<<< HEAD
use crate::actions::insert::*;
=======
>>>>>>> 5ddd46a444d5850df3078e8f6e9f699fe66b83d3

use crate::dtos::SetVoteRequest;
use crate::utils::{AuthenticatedUser, ErrorResponse};
use crate::validators::validate_voting_id;

<<<<<<< HEAD
=======
use diesel::insert_into;
use diesel::prelude::*;
use rocket::http::Status;
>>>>>>> 5ddd46a444d5850df3078e8f6e9f699fe66b83d3
use rocket_contrib::json::Json;

#[put(
    "/votings/<voting_id>/polls/<poll_index>/vote",
    format = "json",
    data = "<input>"
)]
pub fn set_vote(
    conn: DbConn,
    voting_id: String,
    poll_index: i32,
    input: Json<SetVoteRequest>,
    user: AuthenticatedUser,
) -> Result<(), ErrorResponse> {
    validate_voting_id(&voting_id)?;

    let voting =
        find_voting(&conn, &voting_id).and_then(|voting| check_if_voter(&conn, voting, &user))?;
    let poll = find_poll_at_index(&conn, &voting, poll_index)?;
    let voter = find_voter(&conn, &user)?;

<<<<<<< HEAD
    insert_vote(&conn, &poll.id, &voter.id, &input.answer)
=======
    use crate::schema::votes;

    insert_into(votes::table)
        .values((
            votes::poll_fk.eq(&poll.id),
            votes::voter_fk.eq(&voter.id),
            votes::answer.eq(input.answer),
        ))
        .execute(&*conn)
        .map_err(|err| {
            let error_msg = format!(
                "Could not insert vote for poll id: {} and voter id: {} with answer: {:?}",
                poll.id, voter.id, input.answer
            );
            println!("{}. err: {:?}", error_msg, err);
            ErrorResponse {
                reason: error_msg,
                status: Status::InternalServerError,
            }
        })?;

    Ok(())
>>>>>>> 5ddd46a444d5850df3078e8f6e9f699fe66b83d3
}

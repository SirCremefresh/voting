use crate::pool::DbConn;

use crate::actions::check::*;
use crate::actions::find::*;
use crate::actions::insert::*;

use crate::dtos::set_vote_dto;
use crate::utils::{AuthenticatedUser, ErrorResponse};
use crate::validators::validate_voting_id;

use rocket::http::Status;
use rocket_contrib::json::Json;

#[post(
    "/votings/<voting_id>/polls/<poll_index>/vote",
    format = "json",
    data = "<input>"
)]
pub fn set_vote(
    conn: DbConn,
    voting_id: String,
    poll_index: i32,
    input: Json<set_vote_dto::SetVoteRequest>,
    user: AuthenticatedUser,
) -> Result<Json<()>, ErrorResponse> {
    validate_voting_id(&voting_id)?;

    let voting =
        find_voting(&conn, &voting_id).and_then(|voting| check_if_voter(&conn, voting, &user))?;

    if voting.active_poll_index.is_none() {
        return Err(ErrorResponse {
            reason: "Can not vote because no vote is active".to_string(),
            status: Status::BadRequest,
        });
    }
    if voting.active_poll_index.unwrap() != poll_index {
        return Err(ErrorResponse {
            reason: format!(
                "Can not vote because the poll_index: {} is not active",
                poll_index
            ),
            status: Status::BadRequest,
        });
    }

    let poll = find_poll_at_index(&conn, &voting, poll_index)?;
    let voter = find_voter(&conn, &user)?;

    let voted = find_vote(&conn, &poll.id, &voter.id)?;
    if voted.is_some() {
        return Err(ErrorResponse {
            reason: format!(
                "Voter already voted on this poll with index: {}",
                poll_index
            ),
            status: Status::BadRequest,
        });
    }

    insert_vote(&conn, &poll.id, &voter.id, &input.answer)?;
    Ok(Json(()))
}

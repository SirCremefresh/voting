use crate::pool::DbConn;

use crate::actions::check::*;
use crate::actions::find::*;
use crate::actions::insert::*;

use crate::dtos::SetVoteRequest;
use crate::utils::{AuthenticatedUser, ErrorResponse};
use crate::validators::validate_voting_id;

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

    insert_vote(&conn, &poll.id, &voter.id, &input.answer)
}

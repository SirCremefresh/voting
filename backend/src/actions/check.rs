use crate::models::*;
use crate::pool::DbConn;
use crate::utils::{AuthenticatedUser, ErrorResponse};

use crate::actions::find::*;

use rocket::http::Status;

#[inline(always)]
pub fn check_if_voter(
    conn: &DbConn,
    voting: Voting,
    user: &AuthenticatedUser,
) -> Result<Voting, ErrorResponse> {
    let voter = find_voter(conn, &user)?;

    match user.key_hash.to_string().eq(&voter.voter_key_hash) {
        true => Ok(voting),
        false => Err(ErrorResponse {
            reason: format!(
                "Voter is not in voting. User has username: {}",
                voter.username
            ),
            status: Status::Unauthorized,
        }),
    }
}

#[inline(always)]
pub fn check_if_voting_admin(
    voting: Voting,
    user: &AuthenticatedUser,
) -> Result<Voting, ErrorResponse> {
    match user.key_hash.to_string().eq(&voting.admin_key_hash) {
        true => Ok(voting),
        false => Err(ErrorResponse {
            reason: format!("Admin key is not correct for voting with id: {}", voting.id),
            status: Status::Unauthorized,
        }),
    }
}

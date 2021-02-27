use crate::pool::DbConn;

use crate::actions::check::*;
use crate::actions::find::*;
use crate::actions::insert::*;

use crate::dtos::{create_voter_dto, get_voter_info_dto};
use crate::utils::{generate_uuid, hash_string, AuthenticatedUser, ErrorResponse};
use crate::validators::{validate_create_voter_request, validate_voting_id};

use rocket_contrib::json::Json;

#[options("/votings/<voting_id>/voters")]
pub fn cors_create_voter(voting_id: String) -> String {
    format!("/votings/{}/voters", voting_id)
}

#[post("/votings/<voting_id>/voters", format = "json", data = "<input>")]
pub fn create_voter(
    conn: DbConn,
    voting_id: String,
    input: Json<create_voter_dto::CreateVoterRequest>,
    user: AuthenticatedUser,
) -> Result<Json<create_voter_dto::CreateVoterResponse>, ErrorResponse> {
    validate_voting_id(&voting_id)?;
    validate_create_voter_request(&input)?;

    let voter_key = generate_uuid();
    let voter_key_hash = hash_string(&voter_key);

    let voting =
        find_voting(&conn, &voting_id).and_then(|voting| check_if_voting_admin(voting, &user))?;

    insert_voter(&conn, &input.username, &voter_key_hash, &voting.id)?;

    Ok(Json(create_voter_dto::CreateVoterResponse {
        voter_key,
        voting_id: voting.id,
    }))
}

#[options("/votings/<voting_id>/voters/info")]
pub fn cors_get_voter_info(voting_id: String) -> String {
    format!("/votings/{}/voters/info", voting_id)
}

#[get("/votings/<voting_id>/voters/info", format = "json")]
pub fn get_voter_info(
    conn: DbConn,
    voting_id: String,
    user: AuthenticatedUser,
) -> Result<Json<get_voter_info_dto::GetVoterInfoResponse>, ErrorResponse> {
    validate_voting_id(&voting_id)?;

    let voting = find_voting(&conn, &voting_id).and_then(|voting| check_if_voter(&conn, voting, &user))?;

    let voter = find_voter(&conn, &user)?;

    Ok(Json(get_voter_info_dto::GetVoterInfoResponse {
        voting_name: voting.name,
        username: voter.username,
    }))
}

use crate::pool::DbConn;

use crate::actions::check::*;
use crate::actions::find::*;
use crate::actions::get::*;
use crate::actions::insert::*;

use crate::dtos::{CreateVotingRequest, CreateVotingResponse, GetVotingResponse};
use crate::utils::{generate_uuid, hash_string, AuthenticatedUser, ErrorResponse};
use crate::validators::{validate_create_voting_request, validate_voting_id};

use diesel::prelude::*;
use diesel::result::Error;
use rocket::http::Status;
use rocket_contrib::json::Json;

#[post("/votings", format = "json", data = "<input>")]
pub fn create_voting(
    conn: DbConn,
    input: Json<CreateVotingRequest>,
) -> Result<Json<CreateVotingResponse>, ErrorResponse> {
    validate_create_voting_request(&input)?;

    let admin_key = generate_uuid();
    let admin_key_hash = hash_string(&admin_key);

    let voting_id = conn
        .transaction::<String, Error, _>(|| {
            let voting_id = insert_voting(&conn, &input.name, &admin_key_hash)?;

            for (i, poll) in (&input.polls).iter().enumerate() {
                insert_poll(
                    &conn,
                    &poll.name,
                    (i * 10) as i32,
                    &poll.description,
                    &voting_id,
                )?;
            }

            Ok(voting_id)
        })
        .map_err(|err| {
            let error_msg = "Could not insert voting to database".to_string();
            println!("{}. err: {:?}", error_msg, err);
            ErrorResponse {
                reason: error_msg,
                status: Status::InternalServerError,
            }
        })?;

    let create_voting_response = CreateVotingResponse {
        voting_id,
        admin_key,
    };

    Ok(Json(create_voting_response))
}

#[get("/votings/<voting_id>", format = "json")]
pub fn get_voting(
    conn: DbConn,
    voting_id: String,
    user: AuthenticatedUser,
) -> Result<Json<GetVotingResponse>, ErrorResponse> {
    validate_voting_id(&voting_id)?;

    find_voting(&conn, &voting_id)
        .and_then(|voting| check_if_voting_admin(voting, &user))
        .and_then(|voting| get_voting_polls_response_for_voting(conn, voting))
        .map(|(voting, polls_response)| {
            Json(GetVotingResponse {
                voting_id: voting.id,
                name: voting.name,
                polls: polls_response,
            })
        })
}

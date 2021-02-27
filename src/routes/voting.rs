use crate::pool::DbConn;

use crate::actions::check::*;
use crate::actions::find::*;
use crate::actions::insert::*;

use crate::dtos::{create_voting_dto, get_voting_dto};
use crate::utils::{generate_uuid, hash_string, AuthenticatedUser, ErrorResponse};
use crate::validators::{validate_create_voting_request, validate_voting_id};
use crate::models::PollResult;

use diesel::prelude::*;
use diesel::result::Error;
use rocket::http::Status;
use rocket_contrib::json::Json;

#[options("/votings")]
pub fn cors_create_voting() -> String {
    "/votings".to_string()
}

#[post("/votings", format = "json", data = "<input>")]
pub fn create_voting(
    conn: DbConn,
    input: Json<create_voting_dto::CreateVotingRequest>,
) -> Result<Json<create_voting_dto::CreateVotingResponse>, ErrorResponse> {
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

    Ok(Json(create_voting_dto::CreateVotingResponse {
        voting_id,
        admin_key,
    }))
}

#[options("/votings/<voting_id>")]
pub fn cors_get_voting(voting_id: String) -> String {
    format!("/votings/{}", voting_id)
}

#[get("/votings/<voting_id>", format = "json")]
pub fn get_voting(
    conn: DbConn,
    voting_id: String,
    user: AuthenticatedUser,
) -> Result<Json<get_voting_dto::GetVotingResponse>, ErrorResponse> {
    validate_voting_id(&voting_id)?;

    find_voting(&conn, &voting_id)
        .and_then(|voting| check_if_voting_admin(voting, &user))
        .and_then(|voting| {
            Ok((
                get_voting_polls_response(&conn, &voting.id)?,
                find_amount_of_voters(&conn, &voting.id)?,
                voting,
            ))
        })
        .map(|(polls_response, voter_count, voting)| {
            Json(get_voting_dto::GetVotingResponse {
                voting_id: voting.id,
                name: voting.name,
                active_poll_index: voting.active_poll_index,
                polls: polls_response,
                voter_count,
            })
        })
}

fn get_voting_polls_response(
    conn: &DbConn,
    voting_id: &String,
) -> Result<Vec<get_voting_dto::GetVotingPollsResponse>, ErrorResponse> {
    find_poll_results(&conn, &voting_id).map(|loaded_polls| {
        loaded_polls
            .iter()
            .map(|poll| get_voting_dto::GetVotingPollsResponse {
                poll_id: String::from(&*poll.id),
                name: String::from(&*poll.name),
                description: String::from(&*poll.description),
                status: get_status_from_poll(&poll),
                votes_accept: poll.votes_accept,
                votes_decline: poll.votes_decline,
                votes_abstain: poll.votes_abstain,
                votes_total: poll.votes_total,
            })
            .collect::<Vec<get_voting_dto::GetVotingPollsResponse>>()
    })
}

fn get_status_from_poll(poll: &PollResult) -> String {
    if poll.votes_total == 0 {
        return String::from("NOT_VOTED");
    }
    if poll.votes_accept > poll.votes_decline {
        return String::from("ACCEPTED");
    }
    if poll.votes_decline > poll.votes_accept {
        return String::from("DECLINED");
    }
    String::from("DRAW")
}

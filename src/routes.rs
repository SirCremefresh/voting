use super::models::*;
use super::pool::DbConn;

use crate::utils::{generate_uuid, hash_string};
use diesel::insert_into;
use diesel::prelude::*;
use rocket::http::{ContentType, Status};
use rocket::request::Request;
use rocket::response;
use rocket::response::{Responder, Response};
use rocket_contrib::json::Json;
use serde_json::json;
use std::io::Cursor;

#[derive(Serialize, Debug)]
pub struct GetVotingResponse {
    #[serde(rename = "votingId")]
    voting_id: String,
    name: String,
    polls: Vec<GetVotingPollsResponse>,
}

#[derive(Serialize, Debug)]
pub struct GetVotingPollsResponse {
    #[serde(rename = "pollId")]
    poll_id: String,
    name: String,
    description: String,
}

#[derive(Debug)]
pub struct ErrorResponse {
    reason: String,
    status: Status,
}

impl<'r> Responder<'r> for ErrorResponse {
    fn respond_to(self, _: &Request) -> response::Result<'r> {
        Response::build()
            .sized_body(Cursor::new(
                json!({
                    "reason": self.reason,
                    "status": self.status.code
                })
                .to_string(),
            ))
            .status(self.status)
            .header(ContentType::JSON)
            .ok()
    }
}

#[get("/voting/<voting_id>", format = "json")]
pub fn get_voting(
    conn: DbConn,
    voting_id: String,
) -> Result<Json<GetVotingResponse>, ErrorResponse> {
    use super::schema::votings::dsl::votings;

    match votings
        .find(&voting_id)
        .first::<Voting>(&*conn)
        .map(|voting| GetVotingResponse {
            voting_id: voting.id,
            name: voting.name,
            polls: get_voting_polls_response(conn, &voting_id),
        }) {
        Ok(voting) => Ok(Json(voting)),
        Err(_e) => Err(ErrorResponse {
            reason: format!("Voting with id: {} not found.", voting_id),
            status: Status::NotFound,
        }),
    }
}

fn get_voting_polls_response(conn: DbConn, voting_id: &String) -> Vec<GetVotingPollsResponse> {
    use super::schema::polls::dsl::{polls, voting_fk};

    polls
        .filter(voting_fk.eq(&voting_id))
        .load::<Poll>(&*conn)
        .unwrap()
        .iter()
        .map(|poll| GetVotingPollsResponse {
            poll_id: String::from(&*poll.id),
            name: String::from(&*poll.name),
            description: String::from(&*poll.description),
        })
        .collect::<Vec<GetVotingPollsResponse>>()
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct CreateVotingRequest {
    name: String,
    polls: Vec<CreateVotingPollRequest>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct CreateVotingPollRequest {
    name: String,
    description: String,
}

#[derive(Serialize, Debug)]
pub struct CreateVotingResponse {
    #[serde(rename = "votingId")]
    voting_id: String,
    #[serde(rename = "adminKey")]
    admin_key: String,
}

#[post("/votings", format = "json", data = "<input>")]
pub fn create_voting(
    conn: DbConn,
    input: Json<CreateVotingRequest>,
) -> Result<Json<CreateVotingResponse>, ErrorResponse> {
    validate_create_voting_request(&input)?;

    let admin_key = generate_uuid();
    let admin_key_hash = hash_string(&admin_key);

    let voting_id = insert_voting(conn, &input.name, &admin_key_hash);

    let create_voting_response = CreateVotingResponse {
        voting_id,
        admin_key,
    };

    Ok(Json(create_voting_response))
}

fn validate_create_voting_request(input: &Json<CreateVotingRequest>) -> Result<(), ErrorResponse> {
    match input.name.len() {
        5..=60 => Ok(()),
        _ => Err(ErrorResponse {
            reason: "Name length must be between 5 and 60 characters".to_string(),
            status: Status::BadRequest,
        }),
    }?;
    match input.polls.len() {
        1..=100 => Ok(()),
        _ => Err(ErrorResponse {
            reason: "Voting must have between 1 and 100 polls".to_string(),
            status: Status::BadRequest,
        }),
    }
}

fn insert_voting(conn: DbConn, voting_name: &String, voting_admin_key_hash: &String) -> String {
    use super::schema::votings::dsl::{admin_key_hash, id, name, votings};

    let generated_voting_id = insert_into(votings)
        .values((
            name.eq(&voting_name),
            admin_key_hash.eq(&voting_admin_key_hash),
        ))
        .returning(id)
        .get_result(&*conn)
        .unwrap();
    generated_voting_id
}

fn insert_poll(
    conn: DbConn,
    poll_name: &String,
    poll_description: &String,
    poll_voting_fk: &String,
) -> Result<(), ErrorResponse> {
    use super::schema::polls::dsl::{description, id, name, polls, voting_fk};

    match insert_into(polls)
        .values((
            name.eq(&poll_name),
            description.eq(&poll_description),
            voting_fk.eq(&poll_voting_fk),
        ))
        .execute(&*conn)
    {
        Ok(1) => Ok(()),
        _ => Err(ErrorResponse {
            reason: "Could not insert poll".to_string(),
            status: Status::InternalServerError,
        }),
    }
}

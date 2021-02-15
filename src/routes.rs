use super::models::*;
use super::pool::DbConn;
use super::schema::votings::dsl::*;

use crate::utils::{generate_uuid, hash_string};
use diesel::insert_into;
use diesel::prelude::*;
use rocket::response::status::BadRequest;
use rocket_contrib::json::Json;

#[get("/votings")]
pub fn get_votings(conn: DbConn) -> Json<Vec<Voting>> {
    votings
        .limit(69)
        .load::<Voting>(&*conn)
        .map(|xs| Json(xs))
        .unwrap()
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
) -> Result<Json<CreateVotingResponse>, BadRequest<String>> {
    validate_create_voting_request(&input)?;

    let admin_key = generate_uuid();
    let generated_admin_key_hash = hash_string(&admin_key);

    let generated_voting_id = insert_voting(conn, &input.name, &generated_admin_key_hash);

    let create_voting_response = CreateVotingResponse {
        voting_id: generated_voting_id,
        admin_key,
    };

    Ok(Json(create_voting_response))
}

fn validate_create_voting_request(
    input: &Json<CreateVotingRequest>,
) -> Result<(), BadRequest<String>> {
    match input.name.len() {
        5..=60 => Ok(()),
        _ => Err(BadRequest(Some(String::from(
            "Name length must be between 5 and 60 characters",
        )))),
    }?;
    match input.polls.len() {
        1..=100 => Ok(()),
        _ => Err(BadRequest(Some(String::from(
            "It must have between 1 and 100 polls",
        )))),
    }
}

fn insert_voting(conn: DbConn, voting_name: &String, voting_admin_key_hash: &String) -> String {
    let generated_voting_id = insert_into(votings)
        .values((
            name.eq(&voting_name),
            admin_key_hash.eq(&voting_admin_key_hash),
        ))
        .returning(voting_id)
        .get_result(&*conn)
        .unwrap();
    generated_voting_id
}

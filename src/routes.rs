use super::models::*;
use super::pool::DbConn;
use super::schema::votings;
use super::schema::votings::dsl::*;

use diesel::insert_into;
use diesel::prelude::*;
use rocket::logger::error;
use rocket::request::Form;
use rocket::response::status::BadRequest;
use rocket_contrib::json::Json;
use sha2::{Digest, Sha256};
use std::borrow::Borrow;
use std::ptr::null;
use uuid::Uuid;

#[get("/votings")]
pub fn get_votings(conn: DbConn) -> Json<Vec<Voting>> {
    votings
        .limit(69)
        .load::<Voting>(&*conn)
        .map(|xs| Json(xs))
        .unwrap()
}

#[derive(Deserialize, Debug)]
pub struct CreateVotingRequest {
    name: String,
}

#[derive(Serialize, Debug)]
pub struct Poll {
    name: String,
    description: String,
}

#[derive(Serialize, Debug)]
pub struct CreateVotingResponse {
    #[serde(rename = "votingId")]
    voting_id: String,
    #[serde(rename = "adminKey")]
    admin_key: String,
    polls: Vec<Poll>,
}

#[post("/votings", format = "json", data = "<input>")]
pub fn create_voting(
    conn: DbConn,
    input: Json<CreateVotingRequest>,
) -> Result<Json<CreateVotingResponse>, BadRequest<String>> {
    match input.name.len() {
        5...60 => {}
        _ => {
            return Err(BadRequest(Some(String::from(
                "Name length must be between 5 and 60 characters",
            ))));
        }
    };

    let admin_key = String::from(
        Uuid::new_v4()
            .to_hyphenated()
            .encode_lower(&mut Uuid::encode_buffer()),
    );

    let mut hasher = Sha256::new();
    // write input message
    hasher.update(admin_key.as_bytes());

    let generated_admin_key_hash = format!("{:X}", hasher.finalize());

    let generated_voting_id = insert_into(votings)
        .values((
            name.eq(&input.name),
            admin_key_hash.eq(generated_admin_key_hash),
        ))
        .returning(voting_id)
        .get_result(&*conn)
        .unwrap();

    let create_voting_response = CreateVotingResponse {
        voting_id: generated_voting_id,
        admin_key,
        polls: Vec::new(),
    };

    Ok(Json(create_voting_response))
}

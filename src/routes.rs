use super::models::*;
use super::pool::DbConn;
use super::schema::votings::dsl::*;

use crate::utils::{hash_string, generate_uuid};
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
    match input.name.len() {
        5...60 => {}
        _ => {
            return Err(BadRequest(Some(String::from(
                "Name length must be between 5 and 60 characters",
            ))));
        }
    };

    let admin_key = generate_uuid();
    let generated_admin_key_hash = hash_string(&admin_key);

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
    };

    Ok(Json(create_voting_response))
}

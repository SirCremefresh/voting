use super::models::*;
use super::pool::DbConn;
use super::schema::votings;
use super::schema::votings::dsl::*;

use diesel::insert_into;
use diesel::prelude::*;
use rocket::request::Form;
use rocket_contrib::json::Json;
use std::borrow::Borrow;
use std::ptr::null;
use rocket::logger::error;
use rocket::response::status::BadRequest;

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
    name: String, // 5 - 60
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
pub fn create_voting(conn: DbConn, input: Json<CreateVotingRequest>) -> Result<Json<CreateVotingResponse>, BadRequest<String>> {
    match input.name.len() {
        5...60 => {}
        _ => return Err(BadRequest(Some(String::from("Name length must be between 5 and 60 characters"))))
    };

    insert_into(votings)
        .values((name.eq(&input.name)))
        .execute(&*conn)
        .unwrap();

    let create_voting_response = CreateVotingResponse {
        voting_id: String::from("d"),
        admin_key: String::from("a"),
        polls: Vec::new(),
    };

    Ok(Json(create_voting_response))
}

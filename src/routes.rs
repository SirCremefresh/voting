use super::models::*;
use super::pool::DbConn;

use crate::utils::{generate_uuid, hash_string};
use diesel::insert_into;
use diesel::prelude::*;
use rocket::response::status::{BadRequest, NotFound};
use rocket_contrib::json::Json;

#[get("/voting/voring_id", format = "json")]
pub fn get_votings(conn: DbConn) -> Json<Vec<(Poll, Voting)>> {
    use super::schema::polls::dsl::polls;
    use super::schema::votings::dsl::votings;

    polls
        .inner_join(votings)
        .load::<(Poll, Voting)>(&*conn)
        .map(|xs| Json(xs))
        .unwrap()
}

#[get("/voting/voring_id2", format = "json")]
pub fn get_votings2(conn: DbConn) -> Json<Vec<(Voting, Poll)>> {
    use super::schema::polls::dsl::polls;
    use super::schema::votings::dsl::votings;

    votings
        .inner_join(polls)
        .load::<(Voting, Poll)>(&*conn)
        .map(|xs| Json(xs))
        .unwrap()
}

#[get("/voting/<voting_id>", format = "json")]
pub fn get_votings3(conn: DbConn, voting_id: String) -> Result<Json<Vec<(Voting, Poll)>>, NotFound<String>> {
    use super::schema::polls::dsl::polls;
    use super::schema::votings::dsl::votings;

    let voting = match votings
        .find(String::from("asd"))
        .execute(&*conn) {
        Some(voting) => voting,
        None => Err(NotFound(Some(
            format!("Voting with id: {} not found.", voting_id),
        )))
    };
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
    let admin_key_hash = hash_string(&admin_key);

    let voting_id = insert_voting(conn, &input.name, &admin_key_hash);

    let create_voting_response = CreateVotingResponse {
        voting_id,
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
            "Voting must have between 1 and 100 polls",
        )))),
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

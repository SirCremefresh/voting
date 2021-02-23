use super::models::*;
use super::pool::DbConn;

use crate::dtos::{
    CreateVotingRequest, CreateVotingResponse, GetVotingPollsResponse, GetVotingResponse,
};
use crate::utils::{generate_uuid, hash_string, AuthenticatedUser, ErrorResponse};
use crate::validators::validate_create_voting_request;

use diesel::insert_into;
use diesel::prelude::*;
use diesel::result::Error;
use rocket::http::Status;
use rocket::request::Request;
use rocket_contrib::json::Json;

#[catch(401)]
pub fn unauthorized(_req: &Request) -> ErrorResponse {
    ErrorResponse {
        reason: "Could not authenticate user".to_string(),
        status: Status::Unauthorized,
    }
}

#[get("/voting/<voting_id>", format = "json")]
pub fn get_voting(
    conn: DbConn,
    voting_id: String,
    _user: AuthenticatedUser,
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

#[post("/votings", format = "json", data = "<input>")]
pub fn create_voting(
    conn: DbConn,
    input: Json<CreateVotingRequest>,
) -> Result<Json<CreateVotingResponse>, ErrorResponse> {
    validate_create_voting_request(&input)?;

    let admin_key = generate_uuid();
    let admin_key_hash = hash_string(&admin_key);

    let voting_id = match conn.transaction::<String, Error, _>(|| {
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
    }) {
        Ok(voting_id) => Ok(voting_id),
        Err(err) => {
            let error_msg = "Could not insert voting to database".to_string();
            eprintln!("{}. err: {:?}", error_msg, err);
            Err(ErrorResponse {
                reason: error_msg,
                status: Status::InternalServerError,
            })
        }
    }?;

    let create_voting_response = CreateVotingResponse {
        voting_id,
        admin_key,
    };

    Ok(Json(create_voting_response))
}

fn get_voting_polls_response(conn: DbConn, voting_id: &String) -> Vec<GetVotingPollsResponse> {
    use super::schema::polls::dsl::{polls, sequenz_number, voting_fk};

    polls
        .filter(voting_fk.eq(&voting_id))
        .order(sequenz_number.asc())
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

fn insert_voting(
    conn: &DbConn,
    voting_name: &String,
    voting_admin_key_hash: &String,
) -> QueryResult<String> {
    use super::schema::votings::dsl::{admin_key_hash, id, name, votings};

    insert_into(votings)
        .values((
            name.eq(&voting_name),
            admin_key_hash.eq(&voting_admin_key_hash),
        ))
        .returning(id)
        .get_result(&**conn)
}

fn insert_poll(
    conn: &DbConn,
    poll_name: &String,
    poll_sequenz_number: i32,
    poll_description: &String,
    poll_voting_fk: &String,
) -> QueryResult<usize> {
    use super::schema::polls::dsl::{description, name, polls, sequenz_number, voting_fk};

    insert_into(polls)
        .values((
            name.eq(&poll_name),
            sequenz_number.eq(poll_sequenz_number),
            description.eq(&poll_description),
            voting_fk.eq(&poll_voting_fk),
        ))
        .execute(&**conn)
}

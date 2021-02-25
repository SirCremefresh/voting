use super::models::*;
use super::pool::DbConn;

use crate::dtos::{
    CreateVoterRequest, CreateVoterResponse, CreateVotingRequest, CreateVotingResponse,
    GetVotingPollsResponse, GetVotingResponse,
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

#[post("/voting/<voting_id>/voter", format = "json", data = "<input>")]
pub fn create_voter(
    conn: DbConn,
    voting_id: String,
    input: Json<CreateVoterRequest>,
    user: AuthenticatedUser,
) -> Result<Json<CreateVoterResponse>, ErrorResponse> {
    use super::schema::voters;
    use super::schema::votings;

    let voter_key = generate_uuid();
    let voter_key_hash = hash_string(&voter_key);

    let voting = votings::table
        .find(&voting_id)
        .first::<Voting>(&*conn)
        .map_err(|err| match err {
            diesel::NotFound => ErrorResponse {
                reason: format!("Voting with id: {} not found", voting_id),
                status: Status::NotFound,
            },
            err => {
                let error_msg =
                    format!("Could not query database for voting with id: {}", voting_id);
                println!("{}. err: {:?}", error_msg, err);
                ErrorResponse {
                    reason: error_msg,
                    status: Status::InternalServerError,
                }
            }
        })
        .and_then(|voting| check_if_voting_admin(voting, &user))?;

    insert_into(voters::table)
        .values((
            voters::username.eq(&input.username),
            voters::voter_key_hash.eq(&voter_key_hash),
            voters::voting_fk.eq(&voting.id),
        ))
        .execute(&*conn)
        .map_err(|_| ErrorResponse {
            reason: format!("Could not create voter for voting with id: {}.", &voting.id),
            status: Status::InternalServerError,
        })?;

    Ok(Json(CreateVoterResponse {
        voter_key,
        voting_id: voting.id,
    }))
}

fn find_voting(conn: &DbConn, voting_id: &String) -> Result<Voting, ErrorResponse> {
    use super::schema::votings;

    votings::table
        .find(&voting_id)
        .first::<Voting>(&**conn)
        .map_err(|err| match err {
            diesel::NotFound => ErrorResponse {
                reason: format!("Voting with id: {} not found", voting_id),
                status: Status::NotFound,
            },
            err => {
                let error_msg =
                    format!("Could not query database for voting with id: {}", voting_id);
                println!("{}. err: {:?}", error_msg, err);
                ErrorResponse {
                    reason: error_msg,
                    status: Status::InternalServerError,
                }
            }
        })
}

#[get("/voting/<voting_id>", format = "json")]
pub fn get_voting(
    conn: DbConn,
    voting_id: String,
    user: AuthenticatedUser,
) -> Result<Json<GetVotingResponse>, ErrorResponse> {
    use super::schema::votings::dsl::votings;

    votings
        .find(&voting_id)
        .first::<Voting>(&*conn)
        .map_err(|_| ErrorResponse {
            reason: format!("Voting with id: {} not found.", voting_id),
            status: Status::NotFound,
        })
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

fn check_if_voting_admin(
    voting: Voting,
    user: &AuthenticatedUser,
) -> Result<Voting, ErrorResponse> {
    if user.key_hash.to_string() == voting.admin_key_hash {
        Ok(voting)
    } else {
        Err(ErrorResponse {
            reason: format!("Admin key is not correct for voting with id: {}", voting.id),
            status: Status::Unauthorized,
        })
    }
}

fn get_voting_polls_response_for_voting(
    conn: DbConn,
    voting: Voting,
) -> Result<(Voting, Vec<GetVotingPollsResponse>), ErrorResponse> {
    use super::schema::polls::dsl::{polls, sequenz_number, voting_fk};

    let loaded_polls = polls
        .filter(voting_fk.eq(&voting.id))
        .order(sequenz_number.asc())
        .load::<Poll>(&*conn)
        .map_err(|_| ErrorResponse {
            reason: format!("Could not load polls to voting with id: {}", &voting.id),
            status: Status::InternalServerError,
        })
        .map(|loaded_polls| {
            loaded_polls
                .iter()
                .map(|poll| GetVotingPollsResponse {
                    poll_id: String::from(&*poll.id),
                    name: String::from(&*poll.name),
                    description: String::from(&*poll.description),
                })
                .collect::<Vec<GetVotingPollsResponse>>()
        })?;

    Ok((voting, loaded_polls))
}

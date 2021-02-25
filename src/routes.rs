use super::models::*;
use super::pool::DbConn;

use crate::dtos::{
    CreateVoterRequest, CreateVoterResponse, CreateVotingRequest, CreateVotingResponse,
    GetActivePollResponse, GetVoterInfoResponse, GetVotingPollsResponse, GetVotingResponse,
    SetActivePollRequest,
};
use crate::utils::{generate_uuid, hash_string, AuthenticatedUser, ErrorResponse};
use crate::validators::{
    validate_create_voter_request, validate_create_voting_request, validate_voting_id,
};

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

#[post("/votings/<voting_id>/voters", format = "json", data = "<input>")]
pub fn create_voter(
    conn: DbConn,
    voting_id: String,
    input: Json<CreateVoterRequest>,
    user: AuthenticatedUser,
) -> Result<Json<CreateVoterResponse>, ErrorResponse> {
    validate_voting_id(&voting_id)?;
    validate_create_voter_request(&input)?;

    let voter_key = generate_uuid();
    let voter_key_hash = hash_string(&voter_key);

    let voting =
        find_voting(&conn, &voting_id).and_then(|voting| check_if_voting_admin(voting, &user))?;

    insert_voter(&conn, &input.username, &voter_key_hash, &voting.id)?;

    Ok(Json(CreateVoterResponse {
        voter_key,
        voting_id: voting.id,
    }))
}

#[get("/votings/<voting_id>/voters/info", format = "json")]
pub fn get_voter_info(
    conn: DbConn,
    voting_id: String,
    user: AuthenticatedUser,
) -> Result<Json<GetVoterInfoResponse>, ErrorResponse> {
    validate_voting_id(&voting_id)?;

    find_voting(&conn, &voting_id).and_then(|voting| check_if_voter(&conn, voting, &user))?;

    let voter = find_voter(&conn, &user)?;

    Ok(Json(GetVoterInfoResponse {
        username: voter.username,
    }))
}

#[put("/votings/<voting_id>/polls/active", format = "json", data = "<input>")]
pub fn set_active_poll(
    conn: DbConn,
    voting_id: String,
    input: Json<SetActivePollRequest>,
    user: AuthenticatedUser,
) -> Result<(), ErrorResponse> {
    use super::schema::votings;
    validate_voting_id(&voting_id)?;

    let voting =
        find_voting(&conn, &voting_id).and_then(|voting| check_if_voting_admin(voting, &user))?;
    let amount_of_polls = find_amount_of_polls(&conn, &voting)?;

    let poll_index = match input.poll_index {
        Some(poll_index) => {
            if amount_of_polls <= poll_index as i32 {
                return Err(ErrorResponse {
                    reason: "Can not set active poll index larger than the amount of polls"
                        .to_string(),
                    status: Status::BadRequest,
                });
            }
            Some(poll_index as i32)
        }
        None => None,
    };

    diesel::update(&voting)
        .set(votings::active_poll_index.eq(poll_index))
        .execute(&*conn)
        .map_err(|err| {
            let error_msg = format!(
                "Could not set active_poll_index: {:?} for voting with id: {}",
                poll_index, voting_id
            );
            println!("{}. err: {:?}", error_msg, err);
            ErrorResponse {
                reason: error_msg,
                status: Status::InternalServerError,
            }
        })?;

    Ok(())
}

#[get("/votings/<voting_id>/polls/active", format = "json")]
pub fn get_active_poll(
    conn: DbConn,
    voting_id: String,
    user: AuthenticatedUser,
) -> Result<Json<Option<GetActivePollResponse>>, ErrorResponse> {
    validate_voting_id(&voting_id)?;

    let voting =
        find_voting(&conn, &voting_id).and_then(|voting| check_if_voter(&conn, voting, &user))?;

    let active_poll_index = match voting.active_poll_index {
        Some(active_poll_index) => active_poll_index,
        None => return Ok(Json(None)),
    };

    let polls = find_polls(&conn, &voting)?;
    if active_poll_index < 0 || active_poll_index >= polls.len() as i32 {
        println!(
            "Could not load poll for voting with id: {} and active_poll_index: {}",
            &voting_id,
            polls.len()
        );
        return Err(ErrorResponse {
            reason: "Could not load the poll with the current active index".to_string(),
            status: Status::InternalServerError,
        });
    }

    let poll = &polls[active_poll_index as usize];

    Ok(Json(Some(GetActivePollResponse {
        poll_index: active_poll_index,
        name: (&poll.name).to_string(),
        description: (&poll.description).to_string(),
    })))
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

fn check_if_voter(
    conn: &DbConn,
    voting: Voting,
    user: &AuthenticatedUser,
) -> Result<Voting, ErrorResponse> {
    let voter = find_voter(conn, &user)?;

    match user.key_hash.to_string().eq(&voter.voter_key_hash) {
        true => Ok(voting),
        false => Err(ErrorResponse {
            reason: format!(
                "Voter is not in voting. User has username: {}",
                voter.username
            ),
            status: Status::Unauthorized,
        }),
    }
}

fn find_voter(conn: &DbConn, user: &AuthenticatedUser) -> Result<Voter, ErrorResponse> {
    use super::schema::voters;

    voters::table
        .filter(voters::voter_key_hash.eq(&user.key_hash))
        .first::<Voter>(&**conn)
        .map_err(|err| match err {
            diesel::NotFound => ErrorResponse {
                reason: format!("Voter not found with key: REDACTED"),
                status: Status::NotFound,
            },
            err => {
                let error_msg = "Could not query database for voter with key: REDACTED".to_string();
                println!("{}. err: {:?}", error_msg, err);
                ErrorResponse {
                    reason: error_msg,
                    status: Status::InternalServerError,
                }
            }
        })
}

fn check_if_voting_admin(
    voting: Voting,
    user: &AuthenticatedUser,
) -> Result<Voting, ErrorResponse> {
    match user.key_hash.to_string().eq(&voting.admin_key_hash) {
        true => Ok(voting),
        false => Err(ErrorResponse {
            reason: format!("Admin key is not correct for voting with id: {}", voting.id),
            status: Status::Unauthorized,
        }),
    }
}

fn find_amount_of_polls(conn: &DbConn, voting: &Voting) -> Result<i32, ErrorResponse> {
    use super::schema::polls::dsl::{id, polls, voting_fk};
    use diesel::dsl::count;

    polls
        .filter(voting_fk.eq(&voting.id))
        .select(count(id))
        .first::<i64>(&**conn)
        .map_err(|_| ErrorResponse {
            reason: format!(
                "Could not load the amount of polls for voting with id: {}",
                &voting.id
            ),
            status: Status::InternalServerError,
        })
        .map(|polls_count| polls_count as i32)
}

fn find_polls(conn: &DbConn, voting: &Voting) -> Result<Vec<Poll>, ErrorResponse> {
    use super::schema::polls::dsl::{polls, sequenz_number, voting_fk};

    return polls
        .filter(voting_fk.eq(&voting.id))
        .order(sequenz_number.asc())
        .load::<Poll>(&**conn)
        .map_err(|_| ErrorResponse {
            reason: format!("Could not load polls to voting with id: {}", &voting.id),
            status: Status::InternalServerError,
        });
}

fn get_voting_polls_response_for_voting(
    conn: DbConn,
    voting: Voting,
) -> Result<(Voting, Vec<GetVotingPollsResponse>), ErrorResponse> {
    let loaded_polls = find_polls(&conn, &voting).map(|loaded_polls| {
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

fn insert_voter(
    conn: &DbConn,
    username: &String,
    voter_key_hash: &String,
    voting_id: &String,
) -> Result<(), ErrorResponse> {
    use super::schema::voters;

    insert_into(voters::table)
        .values((
            voters::username.eq(&username),
            voters::voter_key_hash.eq(&voter_key_hash),
            voters::voting_fk.eq(&voting_id),
        ))
        .execute(&**conn)
        .map_err(|err| {
            let error_msg = format!("Could not insert voter for voting with id: {}", voting_id);
            println!("{}. err: {:?}", error_msg, err);
            ErrorResponse {
                reason: error_msg,
                status: Status::InternalServerError,
            }
        })?;
    Ok(())
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

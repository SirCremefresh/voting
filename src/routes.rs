use super::actions::*;
use super::models::*;
use super::pool::DbConn;

use crate::dtos::{
    CreateVoterRequest, CreateVoterResponse, CreateVotingRequest, CreateVotingResponse,
    GetActivePollResponse, GetVoterInfoResponse, GetVotingPollsResponse, GetVotingResponse,
    SetActivePollRequest, SetVoteRequest,
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

#[put(
    "/votings/<voting_id>/polls/<poll_index>/vote",
    format = "json",
    data = "<input>"
)]
pub fn set_vote(
    conn: DbConn,
    voting_id: String,
    poll_index: i32,
    input: Json<SetVoteRequest>,
    user: AuthenticatedUser,
) -> Result<(), ErrorResponse> {
    validate_voting_id(&voting_id)?;

    let voting =
        find_voting(&conn, &voting_id).and_then(|voting| check_if_voter(&conn, voting, &user))?;
    let poll = find_poll_at_index(&conn, &voting, poll_index)?;
    let voter = find_voter(&conn, &user)?;

    use super::schema::votes;

    insert_into(votes::table)
        .values((
            votes::poll_fk.eq(&poll.id),
            votes::voter_fk.eq(&voter.id),
            votes::answer.eq(input.answer),
        ))
        .execute(&*conn)
        .map_err(|err| {
            let error_msg = format!(
                "Could not insert vote for poll id: {} and voter id: {} with answer: {:?}",
                poll.id, voter.id, input.answer
            );
            println!("{}. err: {:?}", error_msg, err);
            ErrorResponse {
                reason: error_msg,
                status: Status::InternalServerError,
            }
        })?;

    Ok(())
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

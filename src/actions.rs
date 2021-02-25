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

pub fn insert_voting(
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

pub fn insert_poll(
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

#[inline(always)]
pub fn check_if_voting_admin(
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

#[inline(always)]
pub fn check_if_voter(
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

pub fn find_voter(conn: &DbConn, user: &AuthenticatedUser) -> Result<Voter, ErrorResponse> {
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

pub fn find_poll_at_index(
    conn: &DbConn,
    voting: &Voting,
    index: i32,
) -> Result<Poll, ErrorResponse> {
    use super::schema::polls::dsl::{polls, sequenz_number, voting_fk};

    polls
        .filter(voting_fk.eq(&voting.id))
        .order(sequenz_number.asc())
        .offset(index as i64)
        .first::<Poll>(&**conn)
        .map_err(|err| match err {
            diesel::NotFound => ErrorResponse {
                reason: format!(
                    "Poll at index: {} for voting with id: {} not found",
                    index, &voting.id
                ),
                status: Status::NotFound,
            },
            err => {
                let error_msg = format!(
                    "Could not query database for poll at index: {} for voting with id: {}",
                    index, &voting.id
                );
                println!("{}. err: {:?}", error_msg, err);
                ErrorResponse {
                    reason: error_msg,
                    status: Status::InternalServerError,
                }
            }
        })
}

pub fn find_voting(conn: &DbConn, voting_id: &String) -> Result<Voting, ErrorResponse> {
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

use crate::dtos::{create_voter_dto, create_voting_dto};
use crate::utils::ErrorResponse;

use rocket::http::Status;
use rocket_contrib::json::Json;

pub fn validate_create_voting_request(
    input: &Json<create_voting_dto::CreateVotingRequest>,
) -> Result<(), ErrorResponse> {
    match input.name.len() {
        5..=60 => Ok(()),
        _ => Err(ErrorResponse {
            reason: "Voting Name length must be between 5 and 60 characters".to_string(),
            status: Status::BadRequest,
        }),
    }?;
    match input.polls.len() {
        1..=100 => Ok(()),
        _ => Err(ErrorResponse {
            reason: "Voting must have between 1 and 100 polls".to_string(),
            status: Status::BadRequest,
        }),
    }?;
    validate_create_voting_polls_request(&input.polls)
}

pub fn validate_create_voter_request(
    input: &Json<create_voter_dto::CreateVoterRequest>,
) -> Result<(), ErrorResponse> {
    match input.username.len() {
        5..=60 => Ok(()),
        _ => Err(ErrorResponse {
            reason: "Voter username length must be between 5 and 60 characters".to_string(),
            status: Status::BadRequest,
        }),
    }
}

pub fn validate_voting_id(voting_id: &String) -> Result<(), ErrorResponse> {
    let len = voting_id.len();
    match len {
        36 => Ok(()),
        _ => Err(ErrorResponse {
            reason: format!("Voting id must be of fixed lenght of: 36 was: {}", len),
            status: Status::BadRequest,
        }),
    }
}

fn validate_create_voting_polls_request(
    polls: &Vec<create_voting_dto::CreateVotingPollRequest>,
) -> Result<(), ErrorResponse> {
    for poll in polls {
        match poll.name.len() {
            5..=60 => Ok(()),
            _ => Err(ErrorResponse {
                reason: "Poll Name length must be between 5 and 60 characters".to_string(),
                status: Status::BadRequest,
            }),
        }?;
        match poll.description.len() {
            5..=60 => Ok(()),
            _ => Err(ErrorResponse {
                reason: "Poll Description length must be between 5 and 60 characters".to_string(),
                status: Status::BadRequest,
            }),
        }?;
    }

    Ok(())
}

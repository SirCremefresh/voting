use crate::utils::{ErrorResponse};
use crate::dtos::{CreateVotingRequest, CreateVotingPollRequest};

use rocket::http::Status;
use rocket_contrib::json::Json;

pub fn validate_create_voting_request(input: &Json<CreateVotingRequest>) -> Result<(), ErrorResponse> {
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

fn validate_create_voting_polls_request(polls: &Vec<CreateVotingPollRequest>) -> Result<(), ErrorResponse> {
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
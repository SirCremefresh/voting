use crate::utils::ErrorResponse;

use rocket::http::Status;
use rocket::request::Request;

#[catch(401)]
pub fn unauthorized(_req: &Request) -> ErrorResponse {
    ErrorResponse {
        reason: "Could not authenticate user".to_string(),
        status: Status::Unauthorized,
    }
}

pub mod poll;
pub mod vote;
pub mod voter;
pub mod voting;

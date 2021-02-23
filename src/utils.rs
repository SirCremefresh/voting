use sha2::{Digest, Sha256};
use uuid::Uuid;
use rocket::http::{ContentType, Status};
use rocket::request::Request;
use rocket::response;
use rocket::response::{Responder, Response};
use serde_json::json;
use std::io::Cursor;

pub fn hash_string(string: &String) -> String {
    let mut hasher = Sha256::new();
    hasher.update(string.as_bytes());
    format!("{:X}", hasher.finalize())
}

pub fn generate_uuid() -> String {
    String::from(
        Uuid::new_v4()
            .to_hyphenated()
            .encode_lower(&mut Uuid::encode_buffer()),
    )
}


#[derive(Debug)]
pub struct ErrorResponse {
    pub reason: String,
    pub status: Status,
}

impl<'r> Responder<'r> for ErrorResponse {
    fn respond_to(self, _: &Request) -> response::Result<'r> {
        Response::build()
            .sized_body(Cursor::new(
                json!({
                    "reason": self.reason,
                    "status": self.status.code
                })
                .to_string(),
            ))
            .status(self.status)
            .header(ContentType::JSON)
            .ok()
    }
}
use crate::models::*;
use crate::pool::DbConn;
use crate::utils::{AuthenticatedUser, ErrorResponse};
use diesel::prelude::*;
use rocket::http::Status;

pub fn find_amount_of_polls(conn: &DbConn, voting: &Voting) -> Result<i32, ErrorResponse> {
    use crate::schema::polls::dsl::{id, polls, voting_fk};
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

pub fn find_voter(conn: &DbConn, user: &AuthenticatedUser) -> Result<Voter, ErrorResponse> {
    use crate::schema::voters;

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
    use crate::schema::polls::dsl::{polls, sequenz_number, voting_fk};

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

pub fn find_polls(conn: &DbConn, voting: &Voting) -> Result<Vec<Poll>, ErrorResponse> {
    use crate::schema::polls::dsl::{polls, sequenz_number, voting_fk};

    polls
        .filter(voting_fk.eq(&voting.id))
        .order(sequenz_number.asc())
        .load::<Poll>(&**conn)
        .map_err(|_| ErrorResponse {
            reason: format!("Could not load polls to voting with id: {}", &voting.id),
            status: Status::InternalServerError,
        })
}

pub fn find_voting(conn: &DbConn, voting_id: &String) -> Result<Voting, ErrorResponse> {
    use crate::schema::votings;

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

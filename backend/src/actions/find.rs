use crate::models::*;
use crate::pool::DbConn;
use crate::utils::{AuthenticatedUser, ErrorResponse};
use diesel::prelude::*;
use rocket::http::Status;

pub fn find_amount_of_polls(conn: &DbConn, voting: &Voting) -> Result<i32, ErrorResponse> {
    use crate::schema::polls;
    use diesel::dsl::count;

    polls::table
        .filter(polls::voting_fk.eq(&voting.id))
        .select(count(polls::id))
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

pub fn find_amount_of_voters(conn: &DbConn, voting_id: &String) -> Result<i32, ErrorResponse> {
    use crate::schema::voters;
    use diesel::dsl::count;

    voters::table
        .filter(voters::voting_fk.eq(&voting_id))
        .select(count(voters::id))
        .first::<i64>(&**conn)
        .map_err(|_| ErrorResponse {
            reason: format!(
                "Could not load the amount of polls for voting with id: {}",
                &voting_id
            ),
            status: Status::InternalServerError,
        })
        .map(|voters_count| voters_count as i32)
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
    use crate::schema::polls;

    polls::table
        .filter(polls::voting_fk.eq(&voting.id))
        .order(polls::sequenz_number.asc())
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

pub fn find_polls(conn: &DbConn, voting_id: &String) -> Result<Vec<Poll>, ErrorResponse> {
    use crate::schema::polls;

    polls::table
        .filter(polls::voting_fk.eq(&voting_id))
        .order(polls::sequenz_number.asc())
        .load::<Poll>(&**conn)
        .map_err(|_| ErrorResponse {
            reason: format!("Could not load polls to voting with id: {}", &voting_id),
            status: Status::InternalServerError,
        })
}

pub fn find_poll_results(
    conn: &DbConn,
    voting_id: &String,
) -> Result<Vec<PollResult>, ErrorResponse> {
    use crate::schema_custom::poll_results;

    poll_results::table
        .filter(poll_results::voting_fk.eq(&voting_id))
        .order(poll_results::sequenz_number.asc())
        .load::<PollResult>(&**conn)
        .map_err(|_| ErrorResponse {
            reason: format!("Could not load polls to voting with id: {}", &voting_id),
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

pub fn find_vote(
    conn: &DbConn,
    poll_id: &String,
    voter_id: &String,
) -> Result<Option<Vote>, ErrorResponse> {
    use crate::schema::votes;

    votes::table
        .filter(
            votes::poll_fk
                .eq(&poll_id)
                .and(votes::voter_fk.eq(&voter_id)),
        )
        .first::<Vote>(&**conn)
        .optional()
        .map_err(|err| {
            let error_msg = format!(
                "Could not query database for vote with poll_id: {} and voter_id: {}",
                poll_id, voter_id
            );
            println!("{}. err: {:?}", error_msg, err);
            ErrorResponse {
                reason: error_msg,
                status: Status::InternalServerError,
            }
        })
}

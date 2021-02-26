use crate::pool::DbConn;

use crate::actions::check::*;
use crate::actions::find::*;
<<<<<<< HEAD
use crate::actions::update::*;
=======
>>>>>>> 5ddd46a444d5850df3078e8f6e9f699fe66b83d3

use crate::dtos::{GetActivePollResponse, SetActivePollRequest};
use crate::utils::{AuthenticatedUser, ErrorResponse};
use crate::validators::validate_voting_id;

<<<<<<< HEAD
=======
use diesel::prelude::*;
>>>>>>> 5ddd46a444d5850df3078e8f6e9f699fe66b83d3
use rocket::http::Status;
use rocket_contrib::json::Json;

#[put("/votings/<voting_id>/polls/active", format = "json", data = "<input>")]
pub fn set_active_poll(
    conn: DbConn,
    voting_id: String,
    input: Json<SetActivePollRequest>,
    user: AuthenticatedUser,
) -> Result<(), ErrorResponse> {
<<<<<<< HEAD
=======
    use crate::schema::votings;
>>>>>>> 5ddd46a444d5850df3078e8f6e9f699fe66b83d3
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

<<<<<<< HEAD
    update_voting_active_poll_index(&conn, &voting, &poll_index)
=======
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
>>>>>>> 5ddd46a444d5850df3078e8f6e9f699fe66b83d3
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

    let polls = find_polls(&conn, &voting_id)?;
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

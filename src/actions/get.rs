use crate::pool::DbConn;

use crate::actions::find::*;
use crate::dtos::GetVotingPollsResponse;
use crate::utils::ErrorResponse;

pub fn get_voting_polls_response(
    conn: &DbConn,
    voting_id: &String,
) -> Result<Vec<GetVotingPollsResponse>, ErrorResponse> {
    find_polls(&conn, &voting_id).map(|loaded_polls| {
        loaded_polls
            .iter()
            .map(|poll| GetVotingPollsResponse {
                poll_id: String::from(&*poll.id),
                name: String::from(&*poll.name),
                description: String::from(&*poll.description),
            })
            .collect::<Vec<GetVotingPollsResponse>>()
    })
}

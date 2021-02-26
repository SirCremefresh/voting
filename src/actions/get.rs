use crate::models::*;
use crate::pool::DbConn;

use crate::actions::find::*;
use crate::dtos::GetVotingPollsResponse;
use crate::utils::ErrorResponse;

pub fn get_voting_polls_response_for_voting(
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

#[derive(Serialize, Debug)]
pub struct GetVotingResponse {
    #[serde(rename = "votingId")]
    pub voting_id: String,
    pub name: String,
    pub voter_count: i32,
    pub polls: Vec<GetVotingPollsResponse>,
}

#[derive(Serialize, Debug)]
pub struct GetVotingPollsResponse {
    #[serde(rename = "pollId")]
    pub poll_id: String,
    pub name: String,
    pub description: String,
    #[serde(rename = "votesAccept")]
    pub votes_accept: i64,
    #[serde(rename = "votesDecline")]
    pub votes_decline: i64,
    #[serde(rename = "votesAbstain")]
    pub votes_abstain: i64,
    #[serde(rename = "votesTotal")]
    pub votes_total: i64,
}

#[derive(Serialize, Debug)]
pub struct GetActivePollResponse {
    #[serde(rename = "pollIndex")]
    pub poll_index: i32,
    pub name: String,
    pub description: String,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct CreateVoterRequest {
    pub username: String,
}

#[derive(Serialize, Debug)]
pub struct GetVoterInfoResponse {
    pub username: String,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct SetActivePollRequest {
    #[serde(rename = "pollIndex")]
    pub poll_index: Option<u16>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct SetVoteRequest {
    pub answer: Option<bool>,
}

#[derive(Serialize, Debug)]
pub struct CreateVoterResponse {
    #[serde(rename = "voterKey")]
    pub voter_key: String,

    #[serde(rename = "votingId")]
    pub voting_id: String,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct CreateVotingRequest {
    pub name: String,
    pub polls: Vec<CreateVotingPollRequest>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct CreateVotingPollRequest {
    pub name: String,
    pub description: String,
}

#[derive(Serialize, Debug)]
pub struct CreateVotingResponse {
    #[serde(rename = "votingId")]
    pub voting_id: String,
    #[serde(rename = "adminKey")]
    pub admin_key: String,
}

#[derive(Serialize, Debug)]
pub struct GetVotingResponse {
    #[serde(rename = "votingId")]
    pub voting_id: String,
    pub name: String,
    pub polls: Vec<GetVotingPollsResponse>,
}

#[derive(Serialize, Debug)]
pub struct GetVotingPollsResponse {
    #[serde(rename = "pollId")]
    pub poll_id: String,
    pub name: String,
    pub description: String,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct CreateVoterRequest {
    pub username: String,
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
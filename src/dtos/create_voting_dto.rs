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

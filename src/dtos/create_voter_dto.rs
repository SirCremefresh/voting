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

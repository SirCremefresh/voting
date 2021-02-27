#[derive(Serialize, Debug)]
pub struct GetVoterInfoResponse {
    #[serde(rename = "votingName")]
    pub voting_name: String,
    pub username: String,
}

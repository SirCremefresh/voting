#[derive(Serialize, Debug)]
pub struct GetVotingResponse {
    #[serde(rename = "votingId")]
    pub voting_id: String,
    pub name: String,
    #[serde(rename = "voterCount")]
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

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct SetVoteRequest {
    pub answer: Option<bool>,
}

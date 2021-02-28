#[derive(Serialize, Debug)]
pub struct GetActivePollResponse {
    #[serde(rename = "pollIndex")]
    pub poll_index: i32,
    pub name: String,
    pub description: String,
    pub voted: Option<String>,
}

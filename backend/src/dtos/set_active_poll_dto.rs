#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct SetActivePollRequest {
    #[serde(rename = "pollIndex")]
    pub poll_index: Option<u16>,
}

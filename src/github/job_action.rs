use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum JobAction {
    Queued,
    InProgress,
    Completed,
    Waiting,
}

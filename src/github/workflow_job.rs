use serde::Deserialize;

#[derive(Deserialize)]
pub struct WorkflowJob {
    pub id: u64,
    pub run_id: u64,
    pub name: String,
    pub labels: Vec<String>,
    pub runner_name: Option<String>,
}

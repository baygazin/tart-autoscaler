use serde::Deserialize;

use super::{JobAction, RepositoryRef, WorkflowJob};

#[derive(Deserialize)]
pub struct WorkflowJobEvent {
    pub action: JobAction,
    pub workflow_job: WorkflowJob,
    pub repository: RepositoryRef,
}

use axum::{
    body::Bytes,
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
};

use crate::github::{EVENT_TYPE_HEADER, WORKFLOW_JOB_EVENT, WorkflowJobEvent};

pub async fn workflow_job(headers: HeaderMap, body: Bytes) -> Response {
    let event_type = headers
        .get(EVENT_TYPE_HEADER)
        .and_then(|value| value.to_str().ok())
        .unwrap_or("");

    if event_type != WORKFLOW_JOB_EVENT {
        println!("ignored {event_type} event");
        return StatusCode::ACCEPTED.into_response();
    }

    let event: WorkflowJobEvent = match serde_json::from_slice(&body) {
        Ok(event) => event,
        Err(error) => {
            println!("rejected webhook: {error}");
            return StatusCode::BAD_REQUEST.into_response();
        }
    };

    let job = &event.workflow_job;
    println!(
        "{:?} job {} \"{}\" run {} in {} labels=[{}] runner={}",
        event.action,
        job.id,
        job.name,
        job.run_id,
        event.repository.full_name,
        job.labels.join(", "),
        job.runner_name.as_deref().unwrap_or("-"),
    );

    StatusCode::ACCEPTED.into_response()
}

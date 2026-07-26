use axum::{
    Router, middleware,
    routing::{get, post},
};

use crate::{
    security::{WebhookVerifier, require_valid_signature},
    store::JitTokenStore,
};

use super::{bootstrap, health_check, seed_token, workflow_job};

pub fn build_router(tokens: JitTokenStore, verifier: WebhookVerifier) -> Router {
    Router::new()
        .route("/healthz", get(health_check))
        .route("/internal/v1/tokens", post(seed_token))
        .route("/api/v1/bootstrap", get(bootstrap))
        .route(
            "/webhook",
            post(workflow_job).layer(middleware::from_fn_with_state(
                verifier,
                require_valid_signature,
            )),
        )
        .with_state(tokens)
}

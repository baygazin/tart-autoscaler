use axum::{Json, extract::State, http::StatusCode};

use crate::store::JitTokenStore;

use super::SeedTokenRequest;

pub async fn seed_token(
    State(tokens): State<JitTokenStore>,
    Json(request): Json<SeedTokenRequest>,
) -> StatusCode {
    let pending = tokens.store_token(request.ip, request.jit_config);
    println!("seeded token for {} ({pending} pending)", request.ip);

    StatusCode::CREATED
}

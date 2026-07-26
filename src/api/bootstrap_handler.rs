use axum::{
    extract::{ConnectInfo, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use std::net::SocketAddr;

use crate::store::JitTokenStore;

pub async fn bootstrap(
    ConnectInfo(peer): ConnectInfo<SocketAddr>,
    State(tokens): State<JitTokenStore>,
) -> Response {
    let ip = peer.ip();

    match tokens.take_token(ip) {
        Some(jit_config) => {
            println!("issued token to {ip}, burned");
            (StatusCode::OK, jit_config).into_response()
        }
        None => {
            println!("rejected {ip}: no token");
            StatusCode::NOT_FOUND.into_response()
        }
    }
}

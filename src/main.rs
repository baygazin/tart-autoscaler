use axum::{
    Json, Router,
    extract::{ConnectInfo, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
};
use serde::Deserialize;
use std::{
    collections::HashMap,
    net::{IpAddr, SocketAddr},
    sync::{Arc, Mutex},
};
use tokio::net::TcpListener;

type TokenStore = Arc<Mutex<HashMap<IpAddr, String>>>;

#[derive(Deserialize)]
struct SeedRequest {
    ip: IpAddr,
    jit_config: String,
}

#[tokio::main]
async fn main() {
    let store: TokenStore = Arc::new(Mutex::new(HashMap::new()));

    let app = Router::new()
        .route("/healthz", get(healthz))
        .route("/internal/v1/tokens", post(seed_token))
        .route("/api/v1/bootstrap", get(bootstrap))
        .with_state(store);

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("metadata server listening on http://0.0.0.0:8080");

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}

async fn healthz() -> &'static str {
    "ok"
}

async fn seed_token(
    State(store): State<TokenStore>,
    Json(payload): Json<SeedRequest>,
) -> StatusCode {
    let mut tokens = store.lock().unwrap();
    tokens.insert(payload.ip, payload.jit_config);

    println!("seeded token for {} ({} pending)", payload.ip, tokens.len());

    StatusCode::CREATED
}

async fn bootstrap(
    ConnectInfo(peer): ConnectInfo<SocketAddr>,
    State(store): State<TokenStore>,
) -> Response {
    let ip = peer.ip();
    let token = store.lock().unwrap().remove(&ip);

    match token {
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

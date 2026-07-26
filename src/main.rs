mod api;
mod github;
mod security;
mod store;

use std::net::SocketAddr;
use tokio::net::TcpListener;

use api::build_router;
use security::WebhookVerifier;
use store::JitTokenStore;

#[tokio::main]
async fn main() {
    let webhook_secret =
        std::env::var("GITHUB_WEBHOOK_SECRET").expect("GITHUB_WEBHOOK_SECRET must be set");

    let app = build_router(JitTokenStore::new(), WebhookVerifier::new(webhook_secret));

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("metadata server listening on http://0.0.0.0:8080");

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}

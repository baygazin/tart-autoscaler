use axum::{
    body::Body,
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};

use super::WebhookVerifier;

const MAX_BODY_BYTES: usize = 5 * 1024 * 1024;

pub async fn require_valid_signature(
    State(verifier): State<WebhookVerifier>,
    request: Request,
    next: Next,
) -> Response {
    let (parts, body) = request.into_parts();

    let body = match axum::body::to_bytes(body, MAX_BODY_BYTES).await {
        Ok(body) => body,
        Err(_) => {
            println!("rejected webhook: body larger than {MAX_BODY_BYTES} bytes");
            return StatusCode::PAYLOAD_TOO_LARGE.into_response();
        }
    };

    if let Err(error) = verifier.verify(&parts.headers, &body) {
        println!("rejected webhook: signature {error:?}");
        return StatusCode::UNAUTHORIZED.into_response();
    }

    next.run(Request::from_parts(parts, Body::from(body))).await
}

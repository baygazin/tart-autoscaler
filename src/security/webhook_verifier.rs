use axum::http::HeaderMap;
use hmac::{Hmac, KeyInit, Mac};
use sha2::Sha256;
use std::sync::Arc;

use super::SignatureError;

type HmacSha256 = Hmac<Sha256>;

const SIGNATURE_HEADER: &str = "x-hub-signature-256";
const SIGNATURE_PREFIX: &str = "sha256=";

#[derive(Clone)]
pub struct WebhookVerifier {
    secret: Arc<str>,
}

impl WebhookVerifier {
    pub fn new(secret: String) -> Self {
        Self {
            secret: secret.into(),
        }
    }

    pub fn verify(&self, headers: &HeaderMap, body: &[u8]) -> Result<(), SignatureError> {
        let Some(header) = headers.get(SIGNATURE_HEADER) else {
            return Err(SignatureError::Missing);
        };
        let Ok(signature) = header.to_str() else {
            return Err(SignatureError::Malformed);
        };
        let Some(hex_digest) = signature.strip_prefix(SIGNATURE_PREFIX) else {
            return Err(SignatureError::Malformed);
        };
        let Ok(expected) = hex::decode(hex_digest) else {
            return Err(SignatureError::Malformed);
        };

        let mut mac =
            HmacSha256::new_from_slice(self.secret.as_bytes()).expect("hmac accepts any key size");
        mac.update(body);

        mac.verify_slice(&expected)
            .map_err(|_| SignatureError::Mismatch)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SECRET: &str = "devsecret";
    const BODY: &[u8] = b"{\"action\":\"queued\"}";

    fn headers(signature: &str) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(SIGNATURE_HEADER, signature.parse().unwrap());
        headers
    }

    fn signature_for(secret: &str, body: &[u8]) -> String {
        let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).unwrap();
        mac.update(body);
        format!(
            "{SIGNATURE_PREFIX}{}",
            hex::encode(mac.finalize().into_bytes())
        )
    }

    #[test]
    fn accepts_a_signature_made_with_the_same_secret() {
        let verifier = WebhookVerifier::new(SECRET.to_string());
        let signature = signature_for(SECRET, BODY);

        assert_eq!(verifier.verify(&headers(&signature), BODY), Ok(()));
    }

    #[test]
    fn rejects_a_signature_made_with_another_secret() {
        let verifier = WebhookVerifier::new(SECRET.to_string());
        let signature = signature_for("wrongsecret", BODY);

        assert_eq!(
            verifier.verify(&headers(&signature), BODY),
            Err(SignatureError::Mismatch)
        );
    }

    #[test]
    fn rejects_a_tampered_body() {
        let verifier = WebhookVerifier::new(SECRET.to_string());
        let signature = signature_for(SECRET, BODY);

        assert_eq!(
            verifier.verify(&headers(&signature), b"{\"action\":\"waiting\"}"),
            Err(SignatureError::Mismatch)
        );
    }

    #[test]
    fn rejects_a_missing_header() {
        let verifier = WebhookVerifier::new(SECRET.to_string());

        assert_eq!(
            verifier.verify(&HeaderMap::new(), BODY),
            Err(SignatureError::Missing)
        );
    }

    #[test]
    fn rejects_a_signature_without_the_algorithm_prefix() {
        let verifier = WebhookVerifier::new(SECRET.to_string());
        let signature = signature_for(SECRET, BODY);
        let bare = signature.trim_start_matches(SIGNATURE_PREFIX);

        assert_eq!(
            verifier.verify(&headers(bare), BODY),
            Err(SignatureError::Malformed)
        );
    }

    #[test]
    fn rejects_a_signature_that_is_not_hex() {
        let verifier = WebhookVerifier::new(SECRET.to_string());

        assert_eq!(
            verifier.verify(&headers("sha256=zzzz"), BODY),
            Err(SignatureError::Malformed)
        );
    }
}

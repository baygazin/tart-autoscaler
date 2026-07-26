#[derive(Debug, PartialEq)]
pub enum SignatureError {
    Missing,
    Malformed,
    Mismatch,
}

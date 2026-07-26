use serde::Deserialize;
use std::net::IpAddr;

#[derive(Deserialize)]
pub struct SeedTokenRequest {
    pub ip: IpAddr,
    pub jit_config: String,
}

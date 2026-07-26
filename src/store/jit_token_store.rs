use std::{
    collections::HashMap,
    net::IpAddr,
    sync::{Arc, Mutex},
};

#[derive(Clone, Default)]
pub struct JitTokenStore {
    tokens: Arc<Mutex<HashMap<IpAddr, String>>>,
}

impl JitTokenStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn store_token(&self, ip: IpAddr, jit_config: String) -> usize {
        let mut tokens = self.tokens.lock().unwrap();
        tokens.insert(ip, jit_config);
        tokens.len()
    }

    pub fn take_token(&self, ip: IpAddr) -> Option<String> {
        self.tokens.lock().unwrap().remove(&ip)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;

    fn ip(last_octet: u8) -> IpAddr {
        IpAddr::V4(Ipv4Addr::new(10, 0, 0, last_octet))
    }

    #[test]
    fn returns_a_stored_token_once() {
        let store = JitTokenStore::new();
        store.store_token(ip(5), "jit-config".to_string());

        assert_eq!(store.take_token(ip(5)), Some("jit-config".to_string()));
        assert_eq!(store.take_token(ip(5)), None);
    }

    #[test]
    fn keeps_tokens_of_different_addresses_apart() {
        let store = JitTokenStore::new();
        store.store_token(ip(5), "for-five".to_string());
        store.store_token(ip(6), "for-six".to_string());

        assert_eq!(store.take_token(ip(6)), Some("for-six".to_string()));
        assert_eq!(store.take_token(ip(5)), Some("for-five".to_string()));
    }

    #[test]
    fn overwrites_a_token_stored_for_the_same_address() {
        let store = JitTokenStore::new();
        store.store_token(ip(5), "stale".to_string());
        store.store_token(ip(5), "fresh".to_string());

        assert_eq!(store.take_token(ip(5)), Some("fresh".to_string()));
    }

    #[test]
    fn reports_how_many_tokens_are_pending() {
        let store = JitTokenStore::new();

        assert_eq!(store.store_token(ip(5), "a".to_string()), 1);
        assert_eq!(store.store_token(ip(6), "b".to_string()), 2);
        assert_eq!(store.store_token(ip(6), "b-again".to_string()), 2);
    }
}

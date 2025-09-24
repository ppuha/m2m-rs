use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use serde::Serialize;
use sha2::Sha256;
use std::collections::BTreeMap;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize)]
pub struct Token {
    access_token: String,
    exp: u64,
}

pub fn generate_token(client_id: String) -> Token {
    let key: Hmac<Sha256> = Hmac::new_from_slice(b"some-secret").unwrap();
    let mut claims = BTreeMap::new();
    claims.insert("sub", client_id);
    let current_time = get_time();
    let lifetime = 60 * 60 * 24;
    let expiration_time = current_time + lifetime;
    claims.insert("iat", current_time.to_string());
    claims.insert("exp", expiration_time.to_string());

    let access_token = claims.sign_with_key(&key).unwrap();

    Token {
        access_token: access_token,
        exp: expiration_time,
    }
}

fn get_time() -> u64 {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => 0,
    }
}

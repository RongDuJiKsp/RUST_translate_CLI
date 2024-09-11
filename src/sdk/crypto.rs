use hmac::{Hmac, Mac};
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use serde_json::json;
use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};
use std::fmt::Write;
use hex;
type HmacSha256 = Hmac<Sha256>;
pub struct Crypto;
impl Crypto {
    pub(crate) fn sha256hex(input: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        let result = hasher.finalize();
        hex::encode(result)
    }

    pub(crate) fn hmacsha256(input: &[u8], key:  &[u8]) -> Vec<u8> {
        let mut mac = HmacSha256::new_from_slice(key).expect("HMAC can take key of any size");
        mac.update(input);
        let result = mac.finalize();
        result.into_bytes().to_vec()
    }
}
use hex;
use hmac::{Hmac, Mac};
use sha2::{Digest, Sha256};

pub struct Crypto;
impl Crypto {
    pub fn sha256hex(data: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data);
        hex::encode(hasher.finalize())
    }
    pub fn hmacsha256(data: &[u8], key: &[u8]) -> Vec<u8> {
        let mut mac = Hmac::<Sha256>::new_from_slice(key).expect("HMAC can take key of any size");
        mac.update(data);
        mac.finalize().into_bytes().to_vec()
    }
}
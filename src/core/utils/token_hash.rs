#![allow(dead_code)]
use rand::{thread_rng, Rng};
use sha2::{Digest, Sha256};

pub fn create_random_token() -> String {
    let mut rng = thread_rng();
    // แก้ตรงนี้: ใส่ r# หน้า gen
    let random_bytes: [u8; 32] = rng.r#gen(); 
    hex::encode(random_bytes)
}

pub fn hash_token(token: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(token.as_bytes());
    hex::encode(hasher.finalize())
}
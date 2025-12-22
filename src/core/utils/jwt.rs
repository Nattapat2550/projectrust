use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::config::env::{Env, ENV};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: i32,
    pub email: String,
    // pub name: String, // ❌ ลบออก (pure-api ไม่มี field นี้ใน token)
    pub role: String,
    pub exp: usize,
    pub iat: usize,
}

fn now_ts() -> usize {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as usize
}

/// รองรับค่า JWT_EXPIRES_IN เช่น "30d", "12h", "600" (วินาที)
fn exp_from_env(env: &Env) -> usize {
    let s = env.jwt_expires_in.trim();
    if s.is_empty() {
        return 7 * 24 * 60 * 60;
    }

    let last = s.chars().last().unwrap_or(' ');
    if last.is_ascii_digit() {
        return s.parse::<usize>().unwrap_or(7 * 24 * 60 * 60);
    }

    let num_part = &s[..s.len().saturating_sub(1)];
    let n = num_part.parse::<usize>().unwrap_or(7);

    match last {
        's' => n,
        'm' => n * 60,
        'h' => n * 60 * 60,
        'd' => n * 24 * 60 * 60,
        _ => 7 * 24 * 60 * 60,
    }
}

pub fn sign(
    user_id: i32,
    email: String,
    // name: String, // ❌ ลบออก
    role: String,
    env: &Env,
) -> Result<String, jsonwebtoken::errors::Error> {
    let iat = now_ts();
    let exp = iat + exp_from_env(env);

    let claims = Claims {
        sub: user_id,
        email,
        // name,
        role,
        exp,
        iat,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(env.jwt_secret.as_bytes()),
    )
}

/// ✅ ใหม่: verify โดยรับ secret ตรง ๆ (ไม่พึ่ง ENV global)
pub fn verify_with_secret(
    token: &str,
    jwt_secret: &str,
) -> Result<Claims, jsonwebtoken::errors::Error> {
    let mut validation = Validation::new(Algorithm::HS256);
    validation.validate_exp = true;

    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(jwt_secret.as_bytes()),
        &validation,
    )?;

    Ok(data.claims)
}

/// ✅ ปรับ: เดิม panic ถ้า ENV ไม่ถูก init -> ทำให้ 500
/// ตอนนี้ถ้า ENV ไม่มี จะใช้ secret ว่าง (ผลคือ verify fail -> กลับเป็น 401 ได้)
#[allow(dead_code)]
pub fn verify(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let secret = ENV
        .get()
        .map(|e| e.jwt_secret.as_str())
        .unwrap_or("");

    verify_with_secret(token, secret)
}

use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

type HmacSha256 = Hmac<Sha256>;

const ARGON2_MEMORY_COST: u32 = 65536; // 64 MiB
const ARGON2_PARALLELISM: u32 = 1;
const ARGON2_ITERATIONS: u32 = 3;

pub fn hash_password(password: &str) -> Result<String, AuthError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::new(
        argon2::Algorithm::Argon2id,
        argon2::Version::V0x13,
        argon2::Params::new(
            ARGON2_MEMORY_COST,
            ARGON2_ITERATIONS,
            ARGON2_PARALLELISM,
            None,
        )
        .map_err(|e| AuthError::Argon2(e.to_string()))?,
    );
    let hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| AuthError::Argon2(e.to_string()))?;
    Ok(hash.to_string())
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, AuthError> {
    let parsed_hash = PasswordHash::new(hash).map_err(|e| AuthError::Argon2(e.to_string()))?;
    let argon2 = Argon2::new(
        argon2::Algorithm::Argon2id,
        argon2::Version::V0x13,
        argon2::Params::new(
            ARGON2_MEMORY_COST,
            ARGON2_ITERATIONS,
            ARGON2_PARALLELISM,
            None,
        )
        .map_err(|e| AuthError::Argon2(e.to_string()))?,
    );
    Ok(argon2
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}

pub fn hash_token(token: &str) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(token.as_bytes());
    hex::encode(hasher.finalize())
}

pub fn generate_session_token() -> String {
    Uuid::new_v4().to_string()
}

pub fn hash_otp(otp: &str) -> String {
    hash_token(otp)
}

pub fn generate_otp() -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    format!("{:06}", rng.gen_range(0..1_000_000))
}

pub fn create_hmac_signature(payload: &str, secret: &str) -> Result<String, AuthError> {
    let mut mac = HmacSha256::new_from_slice(secret.as_bytes())
        .map_err(|_| AuthError::Hmac("Invalid secret length".to_string()))?;
    mac.update(payload.as_bytes());
    Ok(hex::encode(mac.finalize().into_bytes()))
}

pub fn verify_hmac_signature(
    payload: &str,
    signature: &str,
    secret: &str,
) -> Result<bool, AuthError> {
    let expected = create_hmac_signature(payload, secret)?;
    Ok(constant_time_eq(&expected, signature))
}

fn constant_time_eq(a: &str, b: &str) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut result = 0u8;
    for (ca, cb) in a.bytes().zip(b.bytes()) {
        result |= ca ^ cb;
    }
    result == 0
}

pub fn current_timestamp() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64
}

#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("Argon2 error: {0}")]
    Argon2(String),
    #[error("HMAC error: {0}")]
    Hmac(String),
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("Session expired")]
    SessionExpired,
    #[error("Session revoked")]
    SessionRevoked,
}

use argon2::{password_hash::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use rand_core::OsRng;

use crate::core::ResultType;
use anyhow::anyhow;
pub async fn argon2_hash(pwd: &str) -> ResultType<String> {
    let pwd_bytes = pwd.as_bytes().to_vec();
    let resp: ResultType<String> = tokio::task::spawn_blocking(move || {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let hashed = argon2
            .hash_password(&pwd_bytes[..], &salt)
            .map_err(|e| anyhow!("Failed to hash: {}", e))?
            .to_string();
        return Ok(hashed);
    })
    .await?;
    return resp;
}
#[allow(dead_code)]
pub async fn argon2_verify(pwd: &str, hash: &str) -> ResultType<bool> {
    let pwd_str = pwd.to_string();
    let hash_str = hash.to_string();
    let resp: ResultType<bool> = tokio::task::spawn_blocking(move || {
        let pwd_bytes = pwd_str.as_bytes();
        let hash_obj = PasswordHash::new(&hash_str)
            .map_err(|e| anyhow!("Invalid password hash: {}, {}", hash_str, e))?;
        return Ok(Argon2::default()
            .verify_password(pwd_bytes, &hash_obj)
            .is_ok());
    })
    .await?;
    return resp;
}

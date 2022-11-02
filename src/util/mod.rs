use std::fmt::Display;

use crate::{config::Config, core::ResultType};
use anyhow::anyhow;
use argon2::{password_hash::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use lettre::{
    transport::smtp::{
        authentication::Credentials,
        client::{Tls, TlsParametersBuilder},
    },
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};
use log::error;
use rand_core::OsRng;
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
// #[allow(dead_code)]
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

pub fn log_ise<T: Display>(e: T) -> actix_web::Error {
    error!("Error: {}", e);
    actix_web::error::ErrorInternalServerError("Internal Server Error")
}

pub async fn simple_sendmail(
    cfg: &Config,
    content: &str,
    subject: &str,
    target: &str,
    receiver_username: &str,
) -> ResultType<()> {
    use encoding_rs::GBK;
    let email = Message::builder()
        .to(format!("{} <{}>", receiver_username, target).parse()?)
        .from(format!("{} <{}>", cfg.common.app_name, cfg.auth.email_sender).parse()?)
        .subject(subject)
        .body(GBK.encode(&content).0.to_vec())
        .map_err(|e| anyhow!("Failed to build email: {}", e))?;
    let use_ssl = cfg.auth.smtp_using_ssl.clone();
    // let smtp_user = cfg.auth.smtp_user.clone();
    // let smtp_pwd = cfg.auth.smtp_password.clone();
    let smtp_server = cfg.auth.smtp_server.clone();
    let smtp_port = cfg.auth.smtp_port.clone();
    let creds = Credentials::new(cfg.auth.smtp_user.clone(), cfg.auth.smtp_password.clone());
    let mailer: AsyncSmtpTransport<Tokio1Executor> = {
        let builder = AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(smtp_server.clone())
            .credentials(creds)
            .port(smtp_port);
        if use_ssl {
            builder
                .tls(Tls::Wrapper(
                    TlsParametersBuilder::new(smtp_server.clone()).build()?,
                ))
                .build()
        } else {
            builder.build()
        }
    };
    mailer
        .send(email)
        .await
        .map_err(|e| anyhow!("Failed to send email: {}", e))?;
    return Ok(());
}

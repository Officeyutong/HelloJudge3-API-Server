use crate::{
    core::{redis_key_email_change, state::HJ3State, ResultType},
    entity::user,
    route::user::model::EmailChangeStoreInfo,
};
use actix_web::{get, web, HttpResponse, Responder};
use anyhow::anyhow;
use log::error;
use redis::AsyncCommands;
use sea_orm::{ActiveModelTrait, Set};
async fn handle_change_email(state: web::Data<HJ3State>, token: String) -> ResultType<String> {
    let mut conn = state.common_cache.get_async_connection().await?;
    let ret_str: Option<String> = conn.get(redis_key_email_change(&token)).await?;

    if let Some(v) = ret_str {
        let decoded = serde_json::from_str::<EmailChangeStoreInfo>(&v)
            .map_err(|e| anyhow!("Failed to decode: {}", e))?;
        use user::*;
        (ActiveModel {
            id: Set(decoded.uid),
            email: Set(decoded.new_email),
            ..Default::default()
        })
        .save(&*state.db)
        .await
        .map_err(|e| anyhow!("Failed to save: {}", e))?;
        return Ok("修改成功！".into());
    } else {
        return Err(anyhow!("口令不正确或已过期！"));
    }
}

#[get("/change_email/{token}")]
pub async fn change_email(
    state: web::Data<HJ3State>,
    info: web::Path<(String,)>,
) -> impl Responder {
    let token = &info.0;
    match handle_change_email(state, token.clone()).await {
        Ok(v) => {
            return HttpResponse::TemporaryRedirect()
                .append_header((
                    "Location",
                    format!("/success?message={}", urlencoding::encode(&v)),
                ))
                .finish()
        }
        Err(e) => {
            error!(
                "Failed to perform email change: {}\n{}",
                e.to_string(),
                e.backtrace()
            );
            return HttpResponse::TemporaryRedirect()
                .append_header((
                    "Location",
                    format!("/error?message={}", urlencoding::encode(&e.to_string())),
                ))
                .finish();
        }
    };
}

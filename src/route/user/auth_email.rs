use crate::{
    core::{
        msg_err_wrp, ok_data_wrp, redis_key_email_auth, state::HJ3State, ActixResult,
        MySimpleResponse, SESSION_LOGIN_TIME, SESSION_UID,
    },
    entity::model::StringList,
    route::user::model::EmailAuthInfo,
    util::log_ise,
};
use actix_session::Session;
use actix_web::{error, post, web};
use anyhow::anyhow;
use log::error;
use redis::AsyncCommands;
use sea_orm::{ActiveModelTrait, Set};
use serde::Deserialize;
use serde_json::json;
#[derive(Deserialize)]
pub struct EmailAuthForm {
    pub username: String,
    pub token: String,
}
#[post("/auth_email")]
pub async fn auth_email(
    session: Session,
    state: web::Data<HJ3State>,
    form: web::Form<EmailAuthForm>,
) -> ActixResult<MySimpleResponse> {
    let cfg = &state.config;
    if cfg.auth.use_phone_when_register_ans_resetpasswd {
        return msg_err_wrp("当前不使用邮箱注册!");
    }
    let mut conn = state
        .common_cache
        .get_async_connection()
        .await
        .map_err(error::ErrorInternalServerError)?;
    let store: Option<String> = conn
        .get(redis_key_email_auth(&form.token))
        .await
        .map_err(|e| {
            error!("Failed to retrive auth cache: {}", e);
            error::ErrorInternalServerError("Redis failed")
        })?;
    if let Some(val) = store {
        let parsed = serde_json::from_str::<EmailAuthInfo>(&val)
            .map_err(|e| {
                anyhow!(
                    "Failed to deserialize email auth info for token {}: {}",
                    form.token,
                    e
                )
            })
            .map_err(log_ise)?;
        if parsed.username != form.username {
            return msg_err_wrp("用户名错误!");
        }
        use crate::entity::user::*;
        let model = ActiveModel {
            username: Set(parsed.username),
            password: Set(parsed.password),
            email: Set(parsed.email),
            register_time: Set(chrono::Local::now().naive_local()),
            description: Set("".into()),
            permissions: Set(StringList(vec![])),
            ..Default::default()
        }
        .save(&*state.db)
        .await
        .map_err(|e| anyhow!("Failed to insert user to db: {}", e))
        .map_err(log_ise)?;
        session
            .insert(SESSION_UID, model.id.clone().unwrap())
            .map_err(log_ise)?;

        session
            .insert(SESSION_LOGIN_TIME, chrono::Local::now().timestamp())
            .map_err(log_ise)?;
        let uid = model.id.unwrap();
        return ok_data_wrp(&json!({ "uid": uid }));
    } else {
        return msg_err_wrp("您的token错误或者已过期!");
    }
}

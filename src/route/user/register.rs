use actix_session::Session;
use actix_web::{post, web};
use log::debug;
use redis::AsyncCommands;
use regex::Regex;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, Set};
use serde::Deserialize;

use crate::{
    core::{
        msg_err_wrp, ok_wrp, redis_key_email_auth, state::HJ3State, ActixResult, MySimpleResponse,
        SESSION_LOGIN_TIME, SESSION_UID,
    },
    route::user::model::EmailAuthInfo,
    util::{argon2_hash, log_ise, simple_sendmail}, entity::model::StringList,
};
use anyhow::anyhow;
#[derive(Deserialize)]
pub struct RegisterForm {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[post("/register")]
pub async fn register(
    state: web::Data<HJ3State>,
    session: Session,
    form: web::Form<RegisterForm>,
) -> ActixResult<MySimpleResponse> {
    let cfg = &state.config;
    if cfg.auth.use_phone_when_register_ans_resetpasswd {
        return msg_err_wrp("当前不使用邮箱注册!");
    }
    if session.get::<i32>(SESSION_UID).map_err(log_ise)?.is_some() {
        return msg_err_wrp("你已登录!");
    }
    let expr = Regex::new(&cfg.common.username_regex)
        .map_err(|e| anyhow!("Invalid username regex: {}", e))
        .map_err(log_ise)?;
    if !expr.is_match(&form.username) {
        return msg_err_wrp(&format!(
            "用户名不合法！用户名需要满足政策表达式: {}",
            cfg.common.username_regex
        ));
    }
    use crate::entity::user::*;
    if Entity::find()
        .filter(Column::Username.eq(form.username.clone()))
        .count(&*state.db)
        .await
        .map_err(|e| anyhow!("Failed to retrive username exists: {}", e))
        .map_err(log_ise)?
        != 0
    {
        return msg_err_wrp("此用户名已被使用!");
    }
    let hashed_password = argon2_hash(&form.password).await.map_err(log_ise)?;
    if cfg.auth.require_register_auth {
        let mut conn = state
            .common_cache
            .get_async_connection()
            .await
            .map_err(log_ise)?;
        let str = serde_json::to_string(&EmailAuthInfo {
            email: form.email.clone(),
            password: hashed_password,
            username: form.username.clone(),
        })
        .map_err(log_ise)?;
        debug!("User cache: {}", str);
        let token = uuid::Uuid::new_v4().to_string();
        conn.set_ex(
            redis_key_email_auth(&token),
            str,
            cfg.auth.register_email_auth_expire_seconds as usize,
        )
        .await
        .map_err(log_ise)?;
        debug!("Redis stored: {}", token);
        let email_body = cfg.auth.register_auth_email.replace("{auth_token}", &token);
        simple_sendmail(cfg, &email_body, "注册", &form.email, "")
            .await
            .map_err(|e| anyhow!("Failed to send email to {}: {}", &form.email, e))
            .map_err(log_ise)?;
        return msg_err_wrp(&format!(
            "验证邮件已经发送到您的邮箱 {} 的垃圾箱，并将于 {} 秒后失效，请注意查收。",
            &form.email, cfg.auth.register_email_auth_expire_seconds
        ));
    } else {
        let model = ActiveModel {
            username: Set(form.username.clone()),
            email: Set(form.email.clone()),
            password: Set(hashed_password),
            register_time: Set(chrono::Local::now().naive_local()),
            description: Set("".into()),
            permissions: Set(StringList(vec![])),
            ..Default::default()
        }
        .insert(&*state.db)
        .await
        .map_err(|e| anyhow!("Failed to insert: {}", e))
        .map_err(log_ise)?;
        let uid = model.id;
        session.insert(SESSION_UID, uid).map_err(log_ise)?;
        session
            .insert(SESSION_LOGIN_TIME, chrono::Local::now().timestamp())
            .map_err(log_ise)?;
        return ok_wrp();
    }
}

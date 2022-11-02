use crate::{
    core::{
        msg_err_wrp, ok_wrp, state::HJ3State, ActixResult, MySimpleResponse, SESSION_LOGIN_TIME,
        SESSION_UID,
    },
    util::{argon2_verify, log_ise},
};
use actix_session::Session;
use actix_web::{
    error, post,
    web::{self},
};
use anyhow::anyhow;
use log::debug;
use sea_orm::{
    ColumnTrait, EntityTrait, FromQueryResult, PaginatorTrait, QueryFilter, QuerySelect,
};
use serde::Deserialize;
#[derive(Deserialize)]
pub struct LoginForm {
    identifier: String,
    password: String,
}
#[post("/login")]
pub async fn login(
    session: Session,
    state: web::Data<HJ3State>,
    form: web::Form<LoginForm>,
) -> ActixResult<MySimpleResponse> {
    let uid = session
        .get::<i32>(SESSION_UID)
        .map_err(|e| anyhow!("Invalid session uid: {}", e))
        .map_err(log_ise)?;
    if uid.is_some() {
        return msg_err_wrp("你已经登录了!");
    }
    use crate::entity::user::*;
    if Entity::find()
        .filter(Column::Email.eq(form.identifier.clone()))
        .limit(2)
        .count(&*state.db)
        .await
        .map_err(error::ErrorInternalServerError)?
        > 1
    {
        return msg_err_wrp("此邮箱对应多个账号，请使用用户名登录!");
    }
    #[derive(FromQueryResult, Debug)]
    struct Local {
        banned: bool,
        password: String,
        id: i32,
    }
    let res = Entity::find()
        .select_only()
        .column(Column::Banned)
        .column(Column::Password)
        .column(Column::Id)
        .filter(
            Column::Email
                .eq(form.identifier.clone())
                .or(Column::Username.eq(form.identifier.clone())),
        )
        .limit(2)
        .into_model::<Local>()
        .all(&*state.db)
        .await
        .map_err(error::ErrorInternalServerError)?;
    debug!("Models found: {:?}", res);
    if res.len() == 0 {
        return msg_err_wrp("用户名或密码错误!");
    }
    if res.len() > 1 {
        return msg_err_wrp("请使用用户名登录！");
    }
    let user = &res[0];
    if user.banned {
        return msg_err_wrp("此账户已被封禁！");
    }
    if !argon2_verify(&form.password, &user.password)
        .await
        .map_err(error::ErrorInternalServerError)?
    {
        return msg_err_wrp("用户名或密码错误!");
    }
    session
        .insert(SESSION_UID, user.id)
        .map_err(error::ErrorInternalServerError)?;
    session
        .insert(SESSION_LOGIN_TIME, chrono::Local::now().timestamp())
        .map_err(error::ErrorInternalServerError)?;
    return ok_wrp();
}

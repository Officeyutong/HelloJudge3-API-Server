use actix_web::{error, post, web};
use log::info;
use redis::AsyncCommands;
use sea_orm::{
    ColumnTrait, EntityTrait, FromQueryResult, PaginatorTrait, QueryFilter, QuerySelect,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    core::{msg_err_wrp, redis_key_reset_password, state::HJ3State, ActixResult, MySimpleResponse},
    route::user::model::ResetPasswordInfo,
    util::{log_ise, simple_sendmail},
};

#[derive(Deserialize)]
pub struct RequireResetPasswordForm {
    identifier: String,
}

#[post("/require_reset_password")]
pub async fn require_reset_password(
    // session: Session,
    state: web::Data<HJ3State>,
    form: web::Form<RequireResetPasswordForm>,
) -> ActixResult<MySimpleResponse> {
    let cfg = &state.config;
    use crate::entity::user::*;
    if Entity::find()
        .filter(Column::Email.eq(form.identifier.clone()))
        .limit(2)
        .count(&*state.db)
        .await
        .map_err(error::ErrorInternalServerError)?
        > 1
    {
        return msg_err_wrp("此邮箱对应多个账号，请使用用户名！");
    }
    #[derive(FromQueryResult)]
    struct Local {
        pub id: i32,
        pub email: String,
    }
    let res = Entity::find()
        .select_only()
        // .column(Column::Banned)
        // .column(Column::Password)
        .column(Column::Id)
        .column(Column::Email)
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
    if res.len() == 0 {
        return msg_err_wrp("用户名或邮箱错误!");
    }
    if res.len() > 1 {
        return msg_err_wrp("请使用用户名!");
    }
    let encoded = serde_json::to_string(&ResetPasswordInfo { uid: res[0].id }).map_err(log_ise)?;
    let uuid = Uuid::new_v4().to_string();
    info!("Reset password info: {}", encoded);

    info!("Token: {}", uuid);
    let mut conn = state
        .common_cache
        .get_async_connection()
        .await
        .map_err(log_ise)?;
    conn.set_ex(
        redis_key_reset_password(&uuid),
        encoded,
        cfg.auth.reset_password_expire_seconds as usize,
    )
    .await
    .map_err(log_ise)?;

    let mail = cfg
        .auth
        .reset_password_email
        .replace("{reset_token}", &uuid);
    simple_sendmail(cfg, &mail, "重置密码", &res[0].email, "")
        .await
        .map_err(log_ise)?;

    return msg_err_wrp(&format!(
        "一封邮件已经发送到您邮箱的垃圾箱，并将在 {} 秒后过期，请注意查收。",
        cfg.auth.reset_password_expire_seconds
    ));
}

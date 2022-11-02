use actix_web::{error, post, web};
use redis::AsyncCommands;
use sea_orm::{
    sea_query::{Expr, IntoTableRef, Query, SimpleExpr},
    ColumnTrait, EntityTrait, FromQueryResult, PaginatorTrait, QueryFilter, QuerySelect, Updater,
    Value,
};
use serde::Deserialize;

use crate::{
    core::{
        msg_err_wrp, msg_ok_wrp, redis_key_reset_password, state::HJ3State, ActixResult,
        MySimpleResponse,
    },
    route::user::model::ResetPasswordInfo,
    util::{argon2_hash, log_ise},
};

#[derive(Deserialize)]
pub struct ResetPasswordForm {
    identifier: String,
    reset_token: String,
    password: String,
}

#[post("/reset_password")]
pub async fn reset_password(
    state: web::Data<HJ3State>,
    form: web::Form<ResetPasswordForm>,
) -> ActixResult<MySimpleResponse> {
    use crate::entity::user::*;
    let cfg = &state.config;
    if cfg.auth.use_phone_when_register_ans_resetpasswd {
        return msg_err_wrp("当前不使用邮箱重置密码!");
    }
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
        // pub email: String,
    }
    let res = Entity::find()
        .select_only()
        // .column(Column::Banned)
        // .column(Column::Password)
        .column(Column::Id)
        // .column(Column::Email)
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
        return msg_err_wrp("用户名错误！");
    }
    if res.len() > 1 {
        return msg_err_wrp("请使用用户名！");
    }

    let mut conn = state
        .common_cache
        .get_async_connection()
        .await
        .map_err(log_ise)?;
    // let val :String = ;
    if let Some(v) = conn
        .get::<_, Option<String>>(redis_key_reset_password(&form.reset_token))
        .await
        .map_err(log_ise)?
    {
        let decoded = serde_json::from_str::<ResetPasswordInfo>(&v).map_err(log_ise)?;
        if decoded.uid != res[0].id {
            return msg_err_wrp("UID错误!");
        }
        let update_stmt = Query::update()
            .table(Entity.into_table_ref())
            .and_where(Expr::col(Column::Id).eq(res[0].id))
            .value_expr(
                Column::ForceLogoutBefore,
                SimpleExpr::Value(Value::BigInt(Some(chrono::Local::now().timestamp()))),
            )
            .value_expr(
                Column::Password,
                SimpleExpr::Value(Value::String(Some(Box::new(
                    argon2_hash(&form.password).await.map_err(log_ise)?,
                )))),
            )
            .to_owned();
        Updater::new(update_stmt)
            .exec(&*state.db)
            .await
            .map_err(log_ise)?;
        return msg_ok_wrp("密码重置完成，请使用新密码登录。");
    } else {
        return msg_err_wrp("Token错误或已过期!");
    }
}

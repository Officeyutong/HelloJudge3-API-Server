use actix_session::Session;
use actix_web::{post, web};
use log::info;
use sea_orm::{
    sea_query::{Expr, IntoTableRef, Query, SimpleExpr},
    Updater, Value,
};

use crate::{
    core::{
        msg_err_wrp, ok_wrp, state::HJ3State, ActixResult, MySimpleResponse, SESSION_LOGIN_TIME,
        SESSION_UID,
    },
    util::log_ise,
};

#[post("/logout")]
pub async fn logout(session: Session, state: web::Data<HJ3State>) -> ActixResult<MySimpleResponse> {
    use crate::entity::user::*;
    if let Some(uid) = session.get::<i32>(SESSION_UID).map_err(log_ise)? {
        session.remove(SESSION_UID);
        session.remove(SESSION_LOGIN_TIME);
        let update_stmt = Query::update()
            .table(Entity.into_table_ref())
            .and_where(Expr::col(Column::Id).eq(uid))
            .value_expr(
                Column::ForceLogoutBefore,
                SimpleExpr::Value(Value::BigInt(Some(chrono::Local::now().timestamp()))),
            )
            .to_owned();
        let r = Updater::new(update_stmt)
            .exec(&*state.db)
            .await
            .map_err(log_ise)?;
        if r.rows_affected == 0 {
            info!("Invalid uid {} logout.", uid);
        }
        return ok_wrp();
    } else {
        return msg_err_wrp("你尚未登录!");
    }
}

use actix_session::Session;
use actix_web::{error::ErrorNotFound, post, web};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, FromQueryResult, QueryFilter, QuerySelect, Set,
};

use crate::{
    core::{session::ParsedSessionState, state::HJ3State, ActixResult, MySimpleRawResponse},
    entity::user,
    util::log_ise,
};
use anyhow::anyhow;
#[post("/toggle_admin_mode")]
pub async fn toggle_admin_mode(
    session: Session,
    state: web::Data<HJ3State>,
) -> ActixResult<MySimpleRawResponse<()>> {
    let db = &*state.db;
    if let Some(uid) = session.uid().ok() {
        if !state
            .perm_manager
            .has_permission(Some(uid), "permission.manage")
            .await
            .map_err(log_ise)?
        {
            return Ok(MySimpleRawResponse::finish_err("你没有权限进行此操作"));
        }
        #[derive(FromQueryResult)]
        struct Local {
            permission_group: String,
        }
        use user::*;
        let Local { permission_group } = Entity::find()
            .select_only()
            .column(Column::PermissionGroup)
            .filter(Column::Id.eq(uid))
            .into_model::<Local>()
            .one(db)
            .await
            .map_err(log_ise)?
            .ok_or_else(|| ErrorNotFound("User not found!"))?;
        let new_perm_group = if permission_group == "default" {
            "admin"
        } else {
            "default"
        };
        (ActiveModel {
            permission_group: Set(new_perm_group.to_string()),
            id: Set(uid),
            ..Default::default()
        })
        .save(db)
        .await
        .map_err(log_ise)?;
        state
            .perm_manager
            .clear_cache(Some(uid))
            .await
            .map_err(|e| anyhow!("Failed to clear user permission cache: {}", e))
            .map_err(log_ise)?;
        return Ok(MySimpleRawResponse::finish_ok(()));
    } else {
        return Ok(MySimpleRawResponse::finish_err("请先登录！"));
    };
}

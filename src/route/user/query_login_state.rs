#![allow(non_snake_case)]

use crate::{
    config::judge_status::JudgeStatusConfig,
    core::{state::HJ3State, SESSION_UID},
    entity::user,
};
use actix_session::Session;
use actix_web::{error, post, web, Responder, Result};
use log::error;
use sea_orm::{EntityTrait, FromQueryResult, QuerySelect};
use serde::Serialize;
// #[derive(Deserialize)]
// pub struct QueryLoginStateInput {
//     withPermission: Option<bool>,
// }
#[derive(Serialize, Default)]
pub struct QueryLoginStateResponse {
    code: i32,
    result: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    uid: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    backend_managable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<String>,
    salt: String,
    judgeStatus: JudgeStatusConfig,
    appName: String,
    usePolling: bool,
    registerURL: String,
    gravatarURL: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    permissions: Option<Vec<String>>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    canUseImageStore: bool,
    displayRepoInFooter: bool,
}
#[post("/query_login_state")]
pub async fn query_login_state(state: web::Data<HJ3State>, session: Session) -> impl Responder {
    return query_login_state_impl(state, session).await;
}
#[post("/this_should_be_the_first_request")]
pub async fn this_should_be_the_first_request(state: web::Data<HJ3State>, session: Session) -> impl Responder {
    return query_login_state_impl(state, session).await;
}

async fn query_login_state_impl(
    // input: Option<web::Json<QueryLoginStateInput>>,
    state: web::Data<HJ3State>,
    session: Session,
) -> Result<web::Json<QueryLoginStateResponse>> {
    let cfg = &state.config;
    let uid_opt = session.get::<i32>(SESSION_UID).unwrap();
    let already_login = uid_opt.is_some();
    let mut result = QueryLoginStateResponse {
        code: 0,
        result: already_login,
        salt: cfg.common.password_salt.clone(),
        judgeStatus: cfg.judge_status.clone(),
        appName: cfg.common.app_name.clone(),
        usePolling: cfg.common.use_polling,
        registerURL: (if cfg.auth.use_phone_when_register_ans_resetpasswd {
            "/phone/register"
        } else {
            "/register"
        })
        .to_string(),
        gravatarURL: cfg.common.gravatar_url_prefix.clone(),
        canUseImageStore: false,
        displayRepoInFooter: cfg.display.display_repo_in_footer,
        ..Default::default()
    };
    if let Some(uid) = uid_opt {
        let user = {
            use user::*;
            #[derive(FromQueryResult)]
            struct Local {
                permission_group: String,
                username: String,
                email: String,
            }
            Entity::find_by_id(uid)
                .select_only()
                .column(Column::PermissionGroup)
                .column(Column::Username)
                .column(Column::Email)
                .into_model::<Local>()
                .one(&*state.db)
                .await
                .map_err(error::ErrorInternalServerError)?
        };
        if let Some(user) = user {
            #[derive(FromQueryResult)]
            struct Local {
                name: String,
            }
            use crate::entity::permission_group::*;
            let perm_grp = Entity::find_by_id(user.permission_group.clone())
                .column(Column::Name)
                .into_model::<Local>()
                .one(&*state.db)
                .await
                .map_err(error::ErrorInternalServerError)?;
            if let Some(grp) = perm_grp {
                result.group = Some(user.permission_group);
                result.group_name = Some(grp.name);
                result.backend_managable = Some(
                    state
                        .perm_manager
                        .has_permission(Some(uid), "backend.manage")
                        .await
                        .map_err(error::ErrorInternalServerError)?,
                );
                result.canUseImageStore = state
                    .perm_manager
                    .has_permission(Some(uid), "imagestore.use")
                    .await
                    .map_err(error::ErrorInternalServerError)?;
                result.username = Some(user.username);
                result.email = Some(user.email);
                result.uid = Some(uid);
            } else {
                error!(
                    "Invalid permission_group {} for user: {}, {}",
                    user.permission_group, uid, user.username
                );
                return Err(error::ErrorNotFound(format!("Invalid permission group!")));
            }
        } else {
            return Err(error::ErrorNotFound(format!("User not found: {}", uid)));
        }
    }
    return Ok(web::Json(result));
}

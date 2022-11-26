use std::collections::HashSet;

use crate::{
    core::{
        msg_ok_wrp, redis_key_email_change, session::ParsedSessionState, state::HJ3State,
        user::user_problem_cache::ensure_accepted_problems_for_user, ActixResult,
        MySimpleRawResponse, MySimpleResponse,
    },
    entity::{
        cached_accepted_problem, contest, follower::following, model::StringList, permission_group,
        team, team_member, user, user_rating_history,
    },
    route::user::model::EmailChangeStoreInfo,
    util::{argon2_hash, log_br, log_ise, simple_sendmail},
};
use actix_session::Session;
use actix_web::{error::ErrorUnauthorized, post, web};
use anyhow::{anyhow, Context};
use log::info;
use redis::AsyncCommands;
use regex::Regex;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, FromQueryResult, QueryFilter, QueryOrder,
    QuerySelect, RelationTrait, Set,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct GetUserProfileRequest {
    uid: i32,
}

#[derive(Serialize)]
pub struct RatingHistoryEntry {
    result: i32,
    contest_id: i32,
    contest_name: String,
}

#[derive(Serialize)]
pub struct JoinedTeamEntry {
    id: i32,
    name: String,
}

#[derive(Serialize)]
pub struct GetUserProfileResponse {
    id: i32,
    banned: i8,
    username: String,
    description: String,
    email: String,
    register_time: String,
    rating: i32,
    rating_history: Vec<RatingHistoryEntry>,
    permission_group: String,
    permissions: Vec<String>,
    phone_verified: bool,
    following: bool,
    phone_number: Option<String>,
    ac_problems: Vec<i32>,
    joined_teams: Vec<JoinedTeamEntry>,
    group_name: String,
    group_permissions: Vec<String>,
    managable: bool,
    #[serde(rename = "canSetAdmin")]
    can_set_admin: bool,
}
#[post("/get_user_profile")]
pub async fn get_user_profile(
    session: Session,
    state: web::Data<HJ3State>,
    form: web::Form<GetUserProfileRequest>,
) -> ActixResult<MySimpleRawResponse<GetUserProfileResponse>> {
    let db = &*state.db;
    let user = user::Entity::find_by_id(form.uid)
        .one(db)
        .await
        .map_err(log_ise)?
        .ok_or(anyhow!("Invalid user id: {}", form.uid))
        .map_err(log_ise)?;
    let self_following = if let Some(self_uid) = session.uid().ok() {
        following(db, self_uid, form.uid).await.map_err(log_ise)?
    } else {
        false
    };
    let phone_number = if user.phone_verified {
        if let Some(val) = user.phone_number {
            Some(
                val.chars()
                    .enumerate()
                    .map(|(idx, v)| if idx <= 2 || idx >= 7 { v } else { '*' })
                    .collect::<String>(),
            )
        } else {
            None
        }
    } else {
        None
    };
    ensure_accepted_problems_for_user(db, state.redis_lock.clone(), &state.config, user.id)
        .await
        .map_err(log_ise)?;

    let ac_problems = {
        #[derive(FromQueryResult)]
        struct Local {
            problem_id: i32,
        }
        use cached_accepted_problem::*;
        Entity::find()
            .select_only()
            .column(Column::ProblemId)
            .filter(Column::Uid.eq(user.id))
            .order_by(Column::ProblemId, sea_orm::Order::Asc)
            .into_model::<Local>()
            .all(db)
            .await
            .with_context(|| format!("Failed to query accepted problems for user {}", user.id))
            .map_err(log_ise)?
            .into_iter()
            .map(|v| v.problem_id)
            .collect::<Vec<_>>()
    };
    let joined_teams = {
        #[derive(FromQueryResult)]
        struct Local {
            team_id: i32,
            name: String,
        }
        team_member::Entity::find()
            .select_only()
            .column(team_member::Column::TeamId)
            .column(team::Column::Name)
            .join(sea_orm::JoinType::Join, team_member::Relation::Team.def())
            .into_model::<Local>()
            .all(db)
            .await
            .with_context(|| format!("Failed to query joined teams for user {}", user.id))
            .map_err(log_ise)?
            .into_iter()
            .map(|v| JoinedTeamEntry {
                id: v.team_id,
                name: v.name,
            })
            .collect::<Vec<_>>()
    };
    let rating_history: Vec<RatingHistoryEntry> = {
        #[derive(FromQueryResult)]
        struct LocalResp {
            value: i32,
            contest_id: i32,
            name: String,
        }
        use user_rating_history::*;
        Entity::find()
            .join(sea_orm::JoinType::Join, Relation::Contest.def())
            .select_only()
            .column(Column::ContestId)
            .column(Column::Value)
            .column(contest::Column::Name)
            .order_by(contest::Column::RatedTime, sea_orm::Order::Desc)
            .into_model::<LocalResp>()
            .all(db)
            .await
            .map_err(log_ise)?
            .into_iter()
            .map(|v| RatingHistoryEntry {
                contest_id: v.contest_id,
                contest_name: v.name,
                result: v.value,
            })
            .collect()
    };
    let (group_name, group_permissions) = if let Some(group) =
        permission_group::Entity::find_by_id(user.permission_group.clone())
            .one(db)
            .await
            .map_err(log_ise)?
    {
        use permission_group::*;
        let mut loaded_group = HashSet::<String>::new();
        let mut group_perms = HashSet::<String>::new();
        let group_name = group.name.clone();
        let mut current_group = Some(group);
        while let Some(ref v) = current_group {
            if loaded_group.contains(&v.id) {
                break;
            }
            group_perms.extend(v.permissions.0.iter().map(|v| v.to_string()));
            loaded_group.insert(v.id.clone());
            if let Some(ref inherit) = v.inherit {
                current_group = Entity::find_by_id(inherit.to_string())
                    .one(db)
                    .await
                    .map_err(log_ise)?;
            } else {
                break;
            }
        }
        (group_name, Vec::from_iter(group_perms.into_iter()))
    } else {
        (
            "<Invalid permission group>".to_string(),
            Vec::<String>::new(),
        )
    };
    let (can_set_admin, managable) = if let Some(self_uid) = session.uid().ok() {
        let perm_manager = &state.perm_manager;
        (
            perm_manager
                .has_permission(Some(self_uid), "permission.manage")
                .await
                .map_err(log_ise)?,
            perm_manager
                .has_permission(Some(self_uid), "user.manage")
                .await
                .map_err(log_ise)?,
        )
    } else {
        (false, false)
    };

    let result = GetUserProfileResponse {
        ac_problems,
        banned: if user.banned { 1 } else { 0 },
        can_set_admin,
        id: user.id,
        username: user.username,
        description: user.description,
        email: user.email,
        register_time: user.register_time.to_string(),
        rating: user.rating,
        rating_history,
        permission_group: user.permission_group,
        permissions: user.permissions.0,
        phone_verified: user.phone_verified,
        following: self_following,
        phone_number,
        joined_teams,
        group_name,
        group_permissions,
        managable,
    };

    return Ok(MySimpleRawResponse::finish_ok(result));
}

#[derive(Deserialize)]
pub struct UpdateProfileRequest {
    uid: i32,
    data: String,
}
#[derive(Deserialize)]
pub struct UpdateProfileForm {
    username: String,
    email: String,
    description: String,
    #[serde(rename = "changePassword")]
    will_change_password: bool,
    #[serde(rename = "newPassword")]
    new_password: String,
    banned: i8,
    // #[serde(rename = "rawAdmin")]
    // raw_admin: bool,
    permissions: Vec<String>,
    permission_group: String,
}
lazy_static::lazy_static! {
    static ref EMAIL_REGEX: Regex = regex::Regex::new(r"(.+)@(.+)").unwrap();
}

#[post("/update_profile")]
pub async fn update_profile(
    session: Session,
    state: web::Data<HJ3State>,
    form: web::Form<UpdateProfileRequest>,
) -> ActixResult<MySimpleResponse> {
    let self_uid = session
        .uid()
        .ok()
        .ok_or_else(|| ErrorUnauthorized("Please login first!"))?;
    let perm_manager = &state.perm_manager;
    let db = &*state.db;
    let target_uid = form.uid;
    use user::*;
    #[derive(FromQueryResult)]
    struct Local {
        permission_group: String,
        permissions: StringList,
        banned: bool,
        email: String,
    }
    let user = match Entity::find()
        .select_only()
        .column(Column::PermissionGroup)
        .column(Column::Permissions)
        .column(Column::Banned)
        .column(Column::Email)
        .into_model::<Local>()
        .one(db)
        .await
        .map_err(log_ise)?
    {
        Some(v) => v,
        None => return Ok(MySimpleResponse::finish_err("用户不存在!")),
    };
    if target_uid != self_uid
        && !perm_manager
            .has_permission(Some(self_uid), "user.manage")
            .await
            .map_err(log_ise)?
    {
        return Ok(MySimpleResponse::finish_err("你无权进行此操作！"));
    }
    let data = serde_json::from_str::<UpdateProfileForm>(&form.data)
        .map_err(|e| anyhow!("数据格式错误: {}", e))
        .map_err(log_br)?;
    let username_regex = regex::Regex::new(&state.config.common.username_regex)
        .map_err(|e| anyhow!("Illegal username regex: {}", e))
        .map_err(log_ise)?;

    if !username_regex.is_match(&data.username) {
        return Ok(MySimpleResponse::finish_err(format!(
            "用户名需要满足正则表达式: {}",
            state.config.common.username_regex
        )));
    }
    if !EMAIL_REGEX.is_match(&data.email) {
        return Ok(MySimpleResponse::finish_err("请输入合法邮箱！"));
    }
    if !perm_manager
        .has_permission(Some(self_uid), "permission.manage")
        .await
        .map_err(log_ise)?
    {
        if data.permission_group != user.permission_group {
            return Ok(MySimpleResponse::finish_err(
                "你没有权限更改用户所属权限组！",
            ));
        }
        if HashSet::<String>::from_iter(data.permissions.clone().into_iter())
            != HashSet::from_iter(user.permissions.0.into_iter())
        {
            return Ok(MySimpleResponse::finish_err("你没有权限更改用户权限！"));
        }
    }
    {
        use permission_group::*;
        if Entity::find_by_id(data.permission_group.clone())
            .one(db)
            .await
            .map_err(log_ise)?
            .is_none()
        {
            return Ok(MySimpleResponse::finish_err(format!(
                "权限组 `{}` 不存在！",
                data.permission_group
            )));
        }
    }
    if user.banned != (data.banned == 1)
        && !perm_manager
            .has_permission(Some(self_uid), "user.manage")
            .await
            .map_err(log_ise)?
    {
        return Ok(MySimpleResponse::finish_err(
            "你没有权限调整用户的封禁状态!",
        ));
    }
    perm_manager
        .clear_cache(Some(target_uid))
        .await
        .map_err(log_ise)?;
    // user.permission_group = data.permission_group;
    // user.permissions  = StringList(data.permissions);
    let email_changed = if data.email != user.email {
        let mut conn = state
            .common_cache
            .get_async_connection()
            .await
            .map_err(log_ise)?;
        let token = uuid::Uuid::new_v4().to_string();

        let email_sec = EmailChangeStoreInfo {
            uid: target_uid,
            new_email: data.email.clone(),
        };
        info!("Change email {}:\n{:#?}", token, email_sec);
        let str = serde_json::to_string(&email_sec).map_err(log_ise)?;
        conn.set_ex(
            redis_key_email_change(&token),
            str,
            state.config.auth.change_email_expire_seconds as usize,
        )
        .await
        .map_err(log_ise)?;
        info!("Stored for {}", token);
        let email_body = state
            .config
            .auth
            .change_email_auth_email
            .replace("{change_token}", &token);
        simple_sendmail(&state.config, &email_body, "更改邮箱", &data.email, "")
            .await
            .map_err(|e| anyhow!("Failed to send email to {}: {}", &data.email, e))
            .map_err(log_ise)?;
        info!("Send email to: {}", data.email);
        true
    } else {
        false
    };
    (ActiveModel {
        id: Set(target_uid),
        permission_group: Set(data.permission_group),
        permissions: Set(StringList(data.permissions)),
        description: Set(data.description),
        banned: Set(data.banned == 1),
        ..(if data.will_change_password {
            ActiveModel {
                password: Set(argon2_hash(&data.new_password).await.map_err(log_ise)?),
                force_logout_before: Set(chrono::Local::now().timestamp()),
                ..Default::default()
            }
        } else {
            Default::default()
        })
    })
    .save(db)
    .await
    .map_err(log_ise)?;
    return msg_ok_wrp(if email_changed {
        format!(
            "您修改了邮箱，一封新的验证邮件已经发送到了 {}，{} 秒内有效。",
            data.email, state.config.auth.change_email_expire_seconds
        )
    } else {
        "修改完成!".into()
    });
}

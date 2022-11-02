use crate::{
    core::{session::ParsedSessionState, state::HJ3State, ActixResult, MySimpleRawResponse},
    entity::{follower::following, user},
    util::log_ise,
};
use actix_session::Session;
use actix_web::{post, web};
use anyhow::anyhow;
use sea_orm::EntityTrait;
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
    banned: bool,
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
    can_set_sdmin: bool,
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
    
    todo!();
}

#![allow(non_snake_case)]
use std::collections::HashSet;

use crate::{
    core::{
        msg_err_wrp, ok_data_msg_wrp, session::ParsedSessionState, state::HJ3State, ActixResult,
        MySimpleResponse,
    },
    entity::user,
    util::log_ise,
};
use actix_session::Session;
use actix_web::{
    post,
    web::{self, Json},
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, DeriveIden, EntityTrait,
    EnumIter, FromQueryResult, ModelTrait, PaginatorTrait, QueryFilter, QuerySelect, RelationTrait,
    Set,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Deserialize)]
pub struct ToggleFollowStateJson {
    pub target: i32,
}

#[derive(Serialize)]
pub struct Followship {
    pub uid: i32,
    pub username: String,
    pub email: String,
    pub followedByMe: bool,
    pub time: i64,
}
#[derive(Serialize)]
pub struct FollowshipResponse {
    pub code: i32,
    pub data: Vec<Followship>,
    pub pageCount: usize,
}
#[post("/toggle_follow_state")]
pub async fn toggle_follow_state(
    session: Session,
    state: web::Data<HJ3State>,
    json: web::Json<ToggleFollowStateJson>,
) -> ActixResult<MySimpleResponse> {
    let uid = session.uid()?;
    let target = json.target;
    if uid == target {
        return msg_err_wrp("禁止关注你自己!");
    }
    use crate::entity::follower::*;
    let query = Entity::find().filter(Column::Source.eq(uid).and(Column::Target.eq(target)));
    let followed = if let Some(val) = query.one(&*state.db).await.map_err(log_ise)? {
        val.delete(&*state.db).await.map_err(log_ise)?;
        false
    } else {
        let total_count = Entity::find()
            .filter(Column::Source.eq(uid))
            .count(&*state.db)
            .await
            .map_err(log_ise)?;
        let max_follow = state.config.common.following_count_limit as usize;
        if total_count >= max_follow {
            return msg_err_wrp(&format!("你最多只能关注 {} 人!", max_follow));
        }
        ActiveModel {
            source: Set(uid),
            target: Set(target),
            time: Set(chrono::Local::now().naive_local()),
        }
        .insert(&*state.db)
        .await
        .map_err(log_ise)?;

        true
    };
    return ok_data_msg_wrp(&json!({ "followed": followed }), "操作完成!");
}

#[derive(Deserialize)]
pub struct GetFollowerListRequest {
    target: i32,
    page: Option<i32>,
}
#[derive(Deserialize)]
pub struct GetFolloweeListRequest {
    source: i32,
    page: Option<i32>,
}

#[post("/get_follower_list")]
async fn get_follower_list(
    session: Session,
    state: web::Data<HJ3State>,
    json: web::Json<GetFollowerListRequest>,
) -> ActixResult<Json<FollowshipResponse>> {
    let self_uid = session.uid().ok();

    let resp = generate_followship_list(
        FollowshipQuery::FollowingSelf(json.target),
        self_uid,
        json.page.unwrap_or(1) as usize,
        state.config.display.followers_per_page as usize,
        &*state.db,
    )
    .await?;
    return Ok(Json(resp));
}
#[post("/get_followee_list")]
async fn get_followee_list(
    session: Session,
    state: web::Data<HJ3State>,
    json: web::Json<GetFolloweeListRequest>,
) -> ActixResult<Json<FollowshipResponse>> {
    let self_uid = session.uid().ok();

    let resp = generate_followship_list(
        FollowshipQuery::SelfFollowing(json.source),
        self_uid,
        json.page.unwrap_or(1) as usize,
        state.config.display.followers_per_page as usize,
        &*state.db,
    )
    .await?;
    return Ok(Json(resp));
}

enum FollowshipQuery {
    SelfFollowing(i32),
    FollowingSelf(i32),
}
#[derive(EnumIter, DeriveIden)]
pub enum ResultCol {
    Target,
    Source,
    Username,
    Email,
    Time,
    SelfFollowed,
}
async fn generate_followship_list(
    query: FollowshipQuery,
    me: Option<i32>,
    page: usize,
    page_size: usize,
    db: &DatabaseConnection,
) -> ActixResult<FollowshipResponse> {
    use crate::entity::follower::*;
    // let f1: DynIden = Arc::new(Alias::new("f1"));
    // let f2: DynIden = Arc::new(Alias::new("f2"));
    // let self_followed: DynIden = Arc::new(Alias::new("self_followed"));
    /*
    这个用户关注的人:
    select
        f1.target,
        user.username,
        (select count(*) from follower as f2 where f2.source=ME and f2.target=f1.target)!=0  as self_followed
        from follower as f1
        join `user` on `user`.id = `target`
        where f1.source = SRC
    关注这个用户的人
    select
        f1.source,
        user.username,
        (select count(*) from follower as f2 where f2.source=ME and f2.target=f1.source)!=0  as self_followed
        from follower as f1
        join `user` on `user`.id = f1.source
        where f1.target = TARGET

    */
    #[derive(FromQueryResult)]
    struct Local {
        uid: i32,
        username: String,
        email: String,
        time: chrono::NaiveDateTime,
    }
    let paginator = Entity::find()
        .select_only()
        .column_as(
            match &query {
                FollowshipQuery::SelfFollowing(_) => Column::Target,
                FollowshipQuery::FollowingSelf(_) => Column::Source,
            },
            "uid",
        )
        .column(user::Column::Username)
        .column(user::Column::Email)
        .column(Column::Time)
        .join(
            sea_orm::JoinType::Join,
            match &query {
                FollowshipQuery::SelfFollowing(_) => Relation::Target.def(),
                FollowshipQuery::FollowingSelf(_) => Relation::Source.def(),
            },
        )
        .filter(match &query {
            FollowshipQuery::SelfFollowing(v) => Column::Source.eq(*v),
            FollowshipQuery::FollowingSelf(v) => Column::Target.eq(*v),
        })
        .into_model::<Local>()
        .paginate(db, page_size);
    let page_count = paginator.num_pages().await.map_err(log_ise)?;
    let current_page = paginator.fetch_page(page - 1).await.map_err(log_ise)?;
    let output = if let Some(self_uid) = me {
        let query_uids = current_page.iter().map(|v| v.uid).collect::<Vec<_>>();
        #[derive(FromQueryResult)]
        struct Local {
            target: i32,
        }

        let followed_by_self = Entity::find()
            .select_only()
            .column(Column::Target)
            .filter(
                Condition::all()
                    .add(Column::Source.eq(self_uid))
                    .add(Column::Target.is_in(query_uids)),
            )
            .into_model::<Local>()
            .all(db)
            .await
            .map_err(log_ise)?
            .into_iter()
            .map(|v| v.target)
            .collect::<HashSet<_>>();
        current_page
            .into_iter()
            .map(|v| Followship {
                uid: v.uid,
                username: v.username,
                email: v.email,
                followedByMe: followed_by_self.contains(&v.uid),
                time: v.time.timestamp(),
            })
            .collect()
    } else {
        current_page
            .into_iter()
            .map(|v| Followship {
                uid: v.uid,
                username: v.username,
                email: v.email,
                followedByMe: false,
                time: v.time.timestamp(),
            })
            .collect()
    };
    return Ok(FollowshipResponse {
        code: 0,
        data: output,
        pageCount: page_count,
    });
}

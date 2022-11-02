#![allow(non_snake_case)]
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
use anyhow::anyhow;
use log::debug;
use sea_orm::{
    sea_query::{
        Alias, BinOper, Expr, IntoIden, IntoTableRef, MysqlQueryBuilder, Query, SimpleExpr,
        SubQueryStatement,
    },
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DeriveIden, DynIden, EntityTrait, EnumIter,
    ModelTrait, PaginatorTrait, QueryFilter, SelectGetableValue, Selector, Set, Value,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;

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
    json: web::Json<GetFollowerListRequest>,
) -> ActixResult<Json<FollowshipResponse>> {
    let self_uid = session.uid().ok();

    let resp = generate_followship_list(
        FollowshipQuery::SelfFollowing(json.target),
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
    let f1: DynIden = Arc::new(Alias::new("f1"));
    let f2: DynIden = Arc::new(Alias::new("f2"));
    let self_followed: DynIden = Arc::new(Alias::new("self_followed"));
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
    let count = Entity::find()
        .filter(match &query {
            FollowshipQuery::SelfFollowing(v) => Column::Source.eq(*v),
            FollowshipQuery::FollowingSelf(v) => Column::Target.eq(*v),
        })
        .count(db)
        .await
        .map_err(|e| anyhow!("Failed to perform database query: {}", e))
        .map_err(log_ise)? as usize;
    let page_count = count / page_size + ((count % page_size != 0) as usize);
    let stmt = Query::select()
        .from_as(Entity.into_table_ref(), f1.clone())
        .column(Column::Target)
        .column(Column::Source)
        .column(user::Column::Username)
        .column(user::Column::Email)
        .column(Column::Time)
        .expr_as(
            match me {
                Some(me) => SimpleExpr::Binary(
                    Box::new(SimpleExpr::SubQuery(Box::new(
                        SubQueryStatement::SelectStatement(
                            Query::select()
                                .from_as(Entity.into_table_ref(), f2.clone())
                                .expr(Expr::asterisk().count())
                                .and_where(
                                    Expr::tbl(f2.clone(), Column::Source.into_iden())
                                        .eq(Value::Int(Some(me)))
                                        .and(
                                            Expr::tbl(f2.clone(), Column::Target.into_iden())
                                                .equals(
                                                    f1.clone(),
                                                    match &query {
                                                        FollowshipQuery::SelfFollowing(_) => {
                                                            Column::Target.into_iden()
                                                        }
                                                        FollowshipQuery::FollowingSelf(_) => {
                                                            Column::Source.into_iden()
                                                        }
                                                    },
                                                ),
                                        ),
                                )
                                .to_owned(),
                        ),
                    ))),
                    BinOper::NotEqual,
                    Box::new(SimpleExpr::Value(Value::Int(Some(0)))),
                ),
                None => SimpleExpr::Value(Value::Bool(Some(false))),
            },
            self_followed.clone(),
        )
        .join(
            sea_orm::JoinType::Join,
            user::Entity.into_table_ref(),
            Expr::tbl(user::Entity.into_iden(), user::Column::Id.into_iden()).equals(
                f1.clone(),
                match &query {
                    FollowshipQuery::SelfFollowing(_) => Column::Target,
                    FollowshipQuery::FollowingSelf(_) => Column::Source,
                },
            ),
        )
        .and_where(match &query {
            FollowshipQuery::SelfFollowing(src_uid) => {
                Expr::tbl(f1.clone(), Column::Source.into_iden()).eq(Value::Int(Some(*src_uid)))
            }
            FollowshipQuery::FollowingSelf(tgt_uid) => {
                Expr::tbl(f1.clone(), Column::Target.into_iden()).eq(Value::Int(Some(*tgt_uid)))
            }
        })
        .limit(page_size as u64)
        .offset((page_size * (page - 1)) as u64)
        .to_owned();
    debug!("Followship query:\n{}", stmt.build(MysqlQueryBuilder).0);

    // <(i32, i32, String, String, chrono::NaiveDateTime, bool)>::find_by_statemenst();
    let entries = Selector::<
        SelectGetableValue<(i32, i32, String, String, chrono::NaiveDateTime, bool), ResultCol>,
    >::with_columns::<(i32, i32, String, String, chrono::NaiveDateTime, bool), ResultCol>(
        stmt
    )
    .all(db)
    .await
    .map_err(log_ise)?;
    let mut output = vec![];
    output.reserve(entries.len());
    for s in entries.into_iter() {
        output.push(Followship {
            uid: match &query {
                FollowshipQuery::SelfFollowing(_) => s.0,
                FollowshipQuery::FollowingSelf(_) => s.1,
            },
            username: s.2,
            email: s.3,
            followedByMe: s.5,
            time: s.4.timestamp(),
        });
    }
    return Ok(FollowshipResponse {
        code: 0,
        data: output,
        pageCount: page_count,
    });
}

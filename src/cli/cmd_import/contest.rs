use std::str::FromStr;

use futures::TryStreamExt;
use log::info;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    QuerySelect, Set,
};
use serde::Deserialize;
use sqlx::{MySqlPool, Row};

use crate::{
    core::ResultType,
    entity::{
        contest::{self, RankCriterion},
        contest_clarification, contest_problem, user_rating_history, virtual_contest,
    },
};
#[derive(Deserialize)]
struct ContestProblem {
    pub id: i32,
    pub weight: f64,
}
#[derive(Deserialize)]
struct RatingHistory {
    pub contest_id: i32,
    pub result: i32,
}
pub async fn import_contest(db: &DatabaseConnection, hj2: &MySqlPool) -> ResultType<()> {
    {
        info!("Importing: contest");
        let mut conn = hj2.acquire().await?;
        let mut rows = sqlx::query("SELECT * FROM contest").fetch(&mut conn);
        while let Some(row) = rows.try_next().await? {
            let cid = row.try_get("id")?;
            let cname: String = row.try_get("name")?;
            contest::ActiveModel {
                id: Set(cid),
                owner_id: Set(row.try_get("owner_id")?),
                name: Set(cname.clone()),
                description: Set(row.try_get("description")?),
                start_time: Set(row.try_get("start_time")?),
                end_time: Set(row.try_get("end_time")?),
                ranklist_visible: Set(row.try_get("ranklist_visible")?),
                judge_result_visible: Set(row.try_get("judge_result_visible")?),
                rank_criterion: Set(RankCriterion::from_str(row.try_get("rank_criterion")?)?),
                invite_code: Set(row.try_get("invite_code")?),
                rated: Set(row.try_get("rated")?),
                rated_time: Set(row.try_get("rated_time")?),
                private_contest: Set(row.try_get("private_contest")?),
                closed: Set(row.try_get("closed")?),
            }
            .insert(db)
            .await?;
            let problems: Vec<ContestProblem> = serde_json::from_str(row.try_get("problems")?)?;
            for (seq, problem) in problems.iter().enumerate() {
                contest_problem::ActiveModel {
                    problem_id: Set(problem.id),
                    contest_id: Set(cid),
                    sequence: Set(seq as i32),
                    score_weight: Set(problem.weight),
                }
                .insert(db)
                .await?;
            }
            info!("Contest done: {}", cname);
        }
    }
    {
        let mut conn = hj2.acquire().await?;
        let mut rows = sqlx::query("SELECT * FROM contest_clarification").fetch(&mut conn);
        while let Some(row) = rows.try_next().await? {
            contest_clarification::ActiveModel {
                id: Set(row.try_get("id")?),
                contest_id: Set(row.try_get("contest")?),
                sender: Set(row.try_get("sender")?),
                send_time: Set(row.try_get("send_time")?),
                content: Set(row.try_get("content")?),
                replier: Set(row.try_get("replier")?),
                reply_content: Set(row.try_get("reply_content")?),
                reply_time: Set(row.try_get("reply_time")?),
            }
            .insert(db)
            .await?;
        }
    }
    {
        let mut conn = hj2.acquire().await?;
        let mut rows =
            sqlx::query("SELECT id,rating_history FROM user WHERE rating_history != '[]'")
                .fetch(&mut conn);
        let mut to_insert = vec![];
        while let Some(row) = rows.try_next().await? {
            let uid: i32 = row.try_get("id")?;
            info!("Update rating history: {}", uid);
            let rating_history: Vec<RatingHistory> =
                serde_json::from_str(row.try_get("rating_history")?)?;
            for item in rating_history.iter() {
                to_insert.push((
                    item.contest_id,
                    user_rating_history::ActiveModel {
                        uid: Set(uid),
                        contest_id: Set(item.contest_id),
                        value: Set(item.result),
                    },
                ));
            }
        }
        for (contest_id, item) in to_insert.into_iter() {
            let count = contest::Entity::find()
                .filter(contest::Column::Id.eq(contest_id))
                .limit(1)
                .count(db)
                .await?;
            if count == 0 {
                info!("Ignore contest: {}", contest_id);
            } else {
                item.insert(db).await?;
            }
        }
    }
    {
        let mut conn = hj2.acquire().await?;
        let mut rows = sqlx::query("SELECT * FROM virtual_contest").fetch(&mut conn);
        while let Some(row) = rows.try_next().await? {
            virtual_contest::ActiveModel {
                id: Set(row.try_get("id")?),
                owner_id: Set(row.try_get("owner_id")?),
                contest_id: Set(row.try_get("contest_id")?),
                start_time: Set(row.try_get("start_time")?),
                end_time: Set(row.try_get("end_time")?),
            }
            .insert(db)
            .await?;
        }
    }
    return Ok(());
}

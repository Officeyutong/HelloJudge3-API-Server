use futures::TryStreamExt;
use log::info;
use sea_orm::{ActiveEnum, ActiveModelTrait, DatabaseConnection, Set};
use sqlx::{MySqlPool, Row};

use crate::{
    core::ResultType,
    entity::{
        model::{SubmissionResult, UsizeList},
        submission::{self, SubmissionStatus},
    },
};
use anyhow::anyhow;
pub async fn import_submission(db: &DatabaseConnection, hj2: &MySqlPool) -> ResultType<()> {
    info!("Importing: submission");
    let models = {
        let mut conn = hj2.acquire().await?;
        let mut rows = sqlx::query("SELECT * FROM submission").fetch(&mut conn);
        let mut curr = vec![];
        while let Some(row) = rows.try_next().await? {
            let id = row.try_get("id")?;
            info!("Submission: {}", id);
            let model = submission::ActiveModel {
                id: Set(id),
                uid: Set(row.try_get("uid")?),
                language: Set(row.try_get("language")?),
                problem_id: Set(row.try_get("problem_id")?),
                submit_time: Set(row.try_get("submit_time")?),
                public: Set(row.try_get("public")?),
                contest_id: Set({
                    let cid: Option<i32> = row.try_get("contest_id")?;
                    if let Some(cid) = cid {
                        if cid == -1 {
                            None
                        } else {
                            Some(cid)
                        }
                    } else {
                        None
                    }
                }),
                virtual_contest_id: Set({
                    let cid: Option<i32> = row.try_get("virtual_contest_id")?;
                    if let Some(cid) = cid {
                        if cid == -1 {
                            None
                        } else {
                            Some(cid)
                        }
                    } else {
                        None
                    }
                }),
                code: Set(row.try_get("code")?),
                judge_result: Set(SubmissionResult(serde_json::from_str(
                    row.try_get("judge_result")?,
                )?)),
                score: Set(row.try_get("score")?),
                memory_cost: Set(row.try_get("memory_cost")?),
                time_cost: Set(row.try_get("time_cost")?),
                extra_compile_parameter: Set(row.try_get("extra_compile_parameter")?),
                selected_compile_parameters: Set(UsizeList(serde_json::from_str(
                    row.try_get("selected_compile_parameters")?,
                )?)),
                status: Set({
                    let status: String = row.try_get("status")?;
                    SubmissionStatus::try_from_value(&status)
                        .map_err(|_| anyhow!("Invalid submission status: {}", status))?
                }),
                message: Set(row.try_get("message")?),
                judger: Set(row.try_get("judger")?),
            };
            curr.push((id, model));
        }
        curr
    };
    for (id, model) in models.into_iter() {
        model.insert(db).await?;
        info!("Submission done: {}", id);
    }
    return Ok(());
}

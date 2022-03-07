use futures::TryStreamExt;
use log::info;
use sea_orm::{ActiveEnum, ActiveModelTrait, DatabaseConnection, Set};
use sqlx::{MySqlPool, Row};

use crate::{
    core::ResultType,
    entity::{
        model::PreliminarySubquestionList,
        preliminary_contest,
        preliminary_problem::{self, PreliminaryProblemType},
    },
};

pub async fn import_preliminary(db: &DatabaseConnection, hj2: &MySqlPool) -> ResultType<()> {
    info!("Importing: preliminary");
    {
        let mut conn = hj2.acquire().await?;
        let mut rows = sqlx::query("SELECT * FROM preliminary_contest").fetch(&mut conn);
        while let Some(row) = rows.try_next().await? {
            let id: i32 = row.try_get("id")?;
            let title: String = row.try_get("title")?;
            info!("Preliminary contest: {}, {}", id, title);
            preliminary_contest::ActiveModel {
                id: Set(id),
                title: Set(title),
                description: Set(row.try_get("description")?),
                uploader: Set(row.try_get("uploader")?),
                duration: Set(row.try_get("duration")?),
                upload_time: Set(row.try_get("upload_time")?),
            }
            .insert(db)
            .await?;
        }
    }
    {
        let mut conn = hj2.acquire().await?;
        let mut rows = sqlx::query("SELECT * FROM preliminary_problem").fetch(&mut conn);
        while let Some(row) = rows.try_next().await? {
            preliminary_problem::ActiveModel {
                id: Set(row.try_get("id")?),
                contest: Set(row.try_get("contest")?),
                problem_type: Set(PreliminaryProblemType::try_from_value(
                    &row.try_get("problem_type")?,
                )?),
                problem_id: Set(row.try_get("problem_id")?),
                content: Set(row.try_get("content")?),
                questions: Set(PreliminarySubquestionList(serde_json::from_str(
                    row.try_get("questions")?,
                )?)),
                score: Set(row.try_get::<f32, &str>("score")? as f64),
            }
            .insert(db)
            .await?;
        }
    }

    return Ok(());
}

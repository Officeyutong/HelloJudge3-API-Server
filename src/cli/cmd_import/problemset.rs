use futures::TryStreamExt;
use log::info;
use sea_orm::{
    ActiveModelTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QuerySelect, Set,
};
use sqlx::{MySqlPool, Row};

use crate::{
    core::ResultType,
    entity::{model::ProblemsetForeignProblem, problem, problemset, problemset_problem},
};
pub async fn import_problemset(db: &DatabaseConnection, hj2: &MySqlPool) -> ResultType<()> {
    info!("Importing: problemset");
    {
        let mut conn = hj2.acquire().await?;
        let mut rows = sqlx::query("SELECT * FROM problem_set").fetch(&mut conn);
        while let Some(row) = rows.try_next().await? {
            let id: i32 = row.try_get("id")?;
            let name: String = row.try_get("name")?;
            info!("Problemset: {}, {}", id, name);
            problemset::ActiveModel {
                id: Set(id),
                name: Set(name.clone()),
                description: Set(row.try_get("description")?),
                owner_id: Set(row.try_get("owner_uid")?),
                create_time: Set(row.try_get("create_time")?),
                private: Set(row.try_get("private")?),
                invite_code: Set(row.try_get("invitation_code")?),
                foreign_problems: Set(ProblemsetForeignProblem(serde_json::from_str(
                    row.try_get("foreign_problems")?,
                )?)),
            }
            .insert(db)
            .await?;
            let problems: Vec<i32> = serde_json::from_str(row.try_get("problems")?)?;
            for (seq, item) in problems.iter().enumerate() {
                if problem::Entity::find_by_id(*item)
                    .limit(1)
                    .count(db)
                    .await?
                    == 0
                {
                    info!("Ignore problem: {}", item);
                    continue;
                }
                problemset_problem::ActiveModel {
                    problemset_id: Set(id),
                    problem_id: Set(*item),
                    sequence: Set(seq as i32),
                }
                .insert(db)
                .await?;
            }
        }
    }
    return Ok(());
}

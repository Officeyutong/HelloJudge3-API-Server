use futures::TryStreamExt;
use log::info;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use sqlx::{MySqlPool, Row};

use crate::{
    core::ResultType,
    entity::{problem_tag, tag},
};

pub async fn import_tag(db: &DatabaseConnection, hj2: &MySqlPool) -> ResultType<()> {
    info!("Importing: tag");
    {
        let mut conn = hj2.acquire().await?;
        let mut rows = sqlx::query("SELECT * FROM tag").fetch(&mut conn);
        while let Some(row) = rows.try_next().await? {
            let id: String = row.try_get("id")?;
            info!("Tag: {}", id);
            tag::ActiveModel {
                id: Set(id),
                display: Set(row.try_get("display")?),
                color: Set(row.try_get("color")?),
            }
            .insert(db) 
            .await?;
        }
    }
    {
        let mut conn = hj2.acquire().await?;
        let mut rows = sqlx::query("SELECT * FROM problemtag").fetch(&mut conn);
        let mut to_insert = vec![];
        while let Some(row) = rows.try_next().await? {
            to_insert.push(problem_tag::ActiveModel {
                problem_id: Set(row.try_get("problem_id")?),
                tag_id: Set(row.try_get("tag_id")?),
            });
        }
        info!("Problem tag to insert: {}", to_insert.len());
        for item in to_insert.into_iter() {
            item.insert(db).await?;
        }
    }
    return Ok(());
}

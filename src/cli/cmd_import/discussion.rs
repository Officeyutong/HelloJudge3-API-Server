use std::str::FromStr;

use futures::TryStreamExt;
use log::info;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use sqlx::{MySqlPool, Row};

use crate::{
    core::{model::discussion_path::DiscussionRoot, ResultType},
    entity::{discussion, discussion_comment},
};
pub async fn import_discussion(db: &DatabaseConnection, hj2: &MySqlPool) -> ResultType<()> {
    info!("Importing: discussion");

    {
        let mut conn = hj2.acquire().await?;
        let mut rows = sqlx::query("SELECT * FROM discussion").fetch(&mut conn);
        while let Some(row) = rows.try_next().await? {
            let cid: i32 = row.try_get("id")?;
            let title: String = row.try_get("title")?;
            info!("Discussion: {}, {}", cid, title);
            discussion::ActiveModel {
                id: Set(cid),
                path: Set(DiscussionRoot::from_str(row.try_get("path")?)?),
                title: Set(title),
                content: Set(row.try_get("content")?),
                uid: Set(row.try_get("uid")?),
                time: Set(row.try_get("time")?),
                private: Set(row.try_get("private")?),
            }
            .insert(db)
            .await?;
        }
    }
    {
        let mut conn = hj2.acquire().await?;
        let mut rows = sqlx::query("SELECT * FROM comment").fetch(&mut conn);
        while let Some(row) = rows.try_next().await? {
            discussion_comment::ActiveModel {
                id: Set(row.try_get("id")?),
                content: Set(row.try_get("content")?),
                uid: Set(row.try_get("uid")?),
                time: Set(row.try_get("time")?),
                discussion_id: Set(row.try_get("discussion_id")?),
            }
            .insert(db)
            .await?;
        }
    }

    return Ok(());
}

use futures::TryStreamExt;
use log::info;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use sqlx::{MySqlPool, Row};

use crate::{core::ResultType, entity::file_storage};

pub async fn import_file(db: &DatabaseConnection, hj2: &MySqlPool) -> ResultType<()> {
    info!("Importing: file_storage");
    let mut conn = hj2.acquire().await?;
    let mut rows = sqlx::query("SELECT * FROM file_storage").fetch(&mut conn);
    while let Some(row) = rows.try_next().await? {
        let file_id: String = row.try_get("uuid")?;
        let filename: String = row.try_get("filename")?;
        info!("File: {}, {}", file_id, filename);
        file_storage::ActiveModel {
            id: Set(file_id),
            name: Set(filename),
            size: Set(row.try_get("filesize")?),
            upload_time: Set(row.try_get("upload_time")?),
        }
        .insert(db)
        .await?;
    }
    return Ok(());
}

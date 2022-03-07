use futures::TryStreamExt;
use log::info;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use sqlx::{MySqlPool, Row};

use crate::{
    core::ResultType,
    entity::{feed, follower, homepage_swiper, image_store, model::StringList, permission_group},
};
pub async fn import_other(db: &DatabaseConnection, hj2: &MySqlPool) -> ResultType<()> {
    info!("Importing: feed, follower, homepage_swiper, image_store, mail, permission_group");
    {
        let mut conn = hj2.acquire().await?;
        let mut rows = sqlx::query("SELECT * FROM feed").fetch(&mut conn);
        while let Some(row) = rows.try_next().await? {
            feed::ActiveModel {
                id: Set(row.try_get("id")?),
                uid: Set(row.try_get("uid")?),
                time: Set(row.try_get("time")?),
                content: Set(row.try_get("content")?),
                top: Set(row.try_get("top")?),
            }
            .insert(db)
            .await?;
        }
    }
    {
        let mut conn = hj2.acquire().await?;
        let mut rows = sqlx::query("SELECT * FROM follower").fetch(&mut conn);
        while let Some(row) = rows.try_next().await? {
            follower::ActiveModel {
                source: Set(row.try_get("source")?),
                target: Set(row.try_get("target")?),
                time: Set(row.try_get("time")?),
            }
            .insert(db)
            .await?;
        }
    }
    {
        let mut conn = hj2.acquire().await?;
        let mut rows = sqlx::query("SELECT * FROM homepage_swiper").fetch(&mut conn);
        while let Some(row) = rows.try_next().await? {
            homepage_swiper::ActiveModel {
                id: Set(row.try_get("id")?),
                image_url: Set(row.try_get("image_url")?),
                link_url: Set(row.try_get("link_url")?),
            }
            .insert(db)
            .await?;
        }
    }
    {
        let mut conn = hj2.acquire().await?;
        let mut rows = sqlx::query("SELECT * FROM image_store_file").fetch(&mut conn);
        let mut to_copy = Vec::<String>::new();
        while let Some(row) = rows.try_next().await? {
            let fid: String = row.try_get("file_id")?;
            image_store::ActiveModel {
                file_id: Set(fid.clone()),
                thumbnail_id: Set(row.try_get("thumbnail_id")?),
                uid: Set(row.try_get("uid")?),
            }
            .insert(db)
            .await?;
            to_copy.push(fid);
        }
        tokio::fs::write("image-store-import.json", serde_json::to_string(&to_copy)?).await?;
    }
    {
        // Not Required
        // let mut conn = hj2.acquire().await?;
        // let mut rows = sqlx::query("SELECT * FROM mail").fetch(&mut conn);
        // while let Some(row) = rows.try_next().await? {
        //     mail::ActiveModel {
        //         id: Set(row.try_get("id")?),
        //         from_id: Set(row.try_get("from_id")?),
        //         to_id: Set(row.try_get("to_id")?),
        //         time: Set(row.try_get("time")?),
        //         text: Set(row.try_get("text")?),
        //     }
        //     .insert(db)
        //     .await?;
        // }
    }
    {
        let mut conn = hj2.acquire().await?;
        let mut rows = sqlx::query("SELECT * FROM permission_group").fetch(&mut conn);
        while let Some(row) = rows.try_next().await? {
            permission_group::ActiveModel {
                id: Set(row.try_get("id")?),
                name: Set(row.try_get("name")?),
                permissions: Set(StringList(serde_json::from_str(
                    row.try_get("permissions")?,
                )?)),
                inherit: Set({
                    let inherit: String = row.try_get("inherit")?;
                    if inherit.is_empty() {
                        None
                    } else {
                        Some(inherit)
                    }
                }),
            }
            .insert(db)
            .await?;
        }
    }

    return Ok(());
}

use futures::TryStreamExt;
use log::info;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use sqlx::{MySqlPool, Row};

use crate::{
    core::ResultType,
    entity::{
        model::wiki_navigation_menu::WikiNavigationMenuList, wiki_config, wiki_navigation_item,
        wiki_page, wiki_page_version,
    },
};
pub async fn import_wiki(db: &DatabaseConnection, hj2: &MySqlPool) -> ResultType<()> {
    info!("Importing: wiki");
    {
        let mut conn = hj2.acquire().await?;
        let mut rows = sqlx::query("SELECT * FROM wikiconfig").fetch(&mut conn);
        while let Some(row) = rows.try_next().await? {
            wiki_config::ActiveModel {
                key: Set(row.try_get("key")?),
                value: Set(row.try_get("value")?),
            }
            .insert(db)
            .await?;
        }
    }
    {
        let mut conn = hj2.acquire().await?;
        let mut rows = sqlx::query("SELECT * FROM wiki_navigation_item").fetch(&mut conn);
        while let Some(row) = rows.try_next().await? {
            wiki_navigation_item::ActiveModel {
                id: Set(row.try_get("id")?),
                title: Set(row.try_get("title")?),
                priority: Set(row.try_get("priority")?),
                menu: Set(WikiNavigationMenuList(serde_json::from_str(
                    row.try_get("menu")?,
                )?)),
            }
            .insert(db)
            .await?;
        }
    }
    {
        let mut conn = hj2.acquire().await?;
        let mut rows = sqlx::query("SELECT * FROM wikipage").fetch(&mut conn);
        while let Some(row) = rows.try_next().await? {
            wiki_page::ActiveModel {
                id: Set(row.try_get("id")?),
                cached_newest_version: Set({
                    let i: i32 = row.try_get("cached_newest_version")?;
                    if i == -1 {
                        None
                    } else {
                        Some(i)
                    }
                }),
            }
            .insert(db)
            .await?;
        }
    }
    {
        let mut conn = hj2.acquire().await?;
        let mut rows = sqlx::query("SELECT * FROM wikipage_version").fetch(&mut conn);
        while let Some(row) = rows.try_next().await? {
            wiki_page_version::ActiveModel {
                id: Set(row.try_get("id")?),
                wikipage_id: Set(row.try_get("wikipage_id")?),
                uid: Set(row.try_get("uid")?),
                title: Set(row.try_get("title")?),
                content: Set(row.try_get("content")?),
                time: Set(row.try_get("time")?),
                verified: Set(row.try_get("verified")?),
                base: Set({
                    let base: i32 = row.try_get("base")?;
                    if base == -1 {
                        None
                    } else {
                        Some(base)
                    }
                }),
                navigation_id: Set(row.try_get("navigation_id")?),
            }
            .insert(db)
            .await?;
        }
    }

    return Ok(());
}

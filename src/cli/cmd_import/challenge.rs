use futures::TryStreamExt;
use log::info;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set, QuerySelect, EntityTrait, PaginatorTrait};
use sqlx::{MySqlPool, Row};

use crate::{
    core::ResultType,
    entity::{challenge, challenge_problemset, challenge_record, problemset},
};
pub async fn import_challenge(db: &DatabaseConnection, hj2: &MySqlPool) -> ResultType<()> {
    info!("Importing: challenge");
    {
        let mut conn = hj2.acquire().await?;
        let mut rows = sqlx::query("SELECT * FROM challenge").fetch(&mut conn);
        while let Some(row) = rows.try_next().await? {
            let cid: i32 = row.try_get("id")?;
            challenge::ActiveModel {
                id: Set(cid),
                name: Set(row.try_get("name")?),
                level: Set(row.try_get("level")?),
                description: Set(row.try_get("description")?),
            }
            .insert(db)
            .await?;
            let problemsets: Vec<i32> = serde_json::from_str(row.try_get("problemset_list")?)?;
            for (seq, item) in problemsets.iter().enumerate() {
                if problemset::Entity::find_by_id(*item)
                    .limit(1)
                    .count(db)
                    .await?
                    != 0
                {
                    challenge_problemset::ActiveModel {
                        challenge_id: Set(cid),
                        problemset_id: Set(*item),
                        sequence: Set(seq as i32),
                    }
                    .insert(db)
                    .await?;
                } else {
                    info!("Ignore problemset: {}", item);
                }
            }
        }
    }
    {
        let mut conn = hj2.acquire().await?;
        let mut rows = sqlx::query("SELECT * FROM challenge_record").fetch(&mut conn);
        while let Some(row) = rows.try_next().await? {
            challenge_record::ActiveModel {
                uid: Set(row.try_get("uid")?),
                challenge_id: Set(row.try_get("challenge_id")?),
                problemset_id: Set(row.try_get("problemset_id")?),
                finished: Set(row.try_get("finished")?),
            }
            .insert(db)
            .await?;
        }
    }

    return Ok(());
}

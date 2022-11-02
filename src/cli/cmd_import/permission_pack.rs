use futures::TryStreamExt;
use log::{debug, info};
use sea_orm::{
    ActiveModelTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QuerySelect, Set,
};
use sqlx::{MySqlPool, Row};

use crate::{
    core::ResultType,
    entity::{model::StringList, permission_pack, permission_pack_user},
};

pub async fn import_permission_pack(db: &DatabaseConnection, hj2: &MySqlPool) -> ResultType<()> {
    info!("Importing: permission_pack");
    {
        let mut conn = hj2.acquire().await?;
        let mut rows = sqlx::query("SELECT * FROM permission_pack").fetch(&mut conn);
        while let Some(row) = rows.try_next().await? {
            permission_pack::ActiveModel {
                id: Set(row.try_get("id")?),
                name: Set(row.try_get("name")?),
                description: Set({
                    let v: Option<String> = row.try_get("description")?;
                    v.unwrap_or("".to_string())
                }),
                permissions: Set(StringList(serde_json::from_str(
                    row.try_get("permissions")?,
                )?)),
            }
            .insert(db)
            .await?;
        }
    }
    {
        let mut conn = hj2.acquire().await?;
        let mut rows = sqlx::query("SELECT * FROM permission_pack_user").fetch(&mut conn);
        while let Some(row) = rows.try_next().await? {
            let pack_id = row.try_get("pack_id")?;
            let phone: String = row.try_get("phone")?;
            debug!("Insert permission_pack_user: {}, {}", pack_id, phone);
            if permission_pack_user::Entity::find_by_id((pack_id, phone.clone()))
                .limit(1)
                .count(db)
                .await?
                == 0
            {
                permission_pack_user::ActiveModel {
                    pack_id: Set(pack_id),
                    phone: Set(phone),
                    claimed: Set(row.try_get("claimed")?),
                }
                .insert(db)
                .await?;
            } else {
                info!("Ignore permission_pack_user: {}, {}", pack_id, phone);
            }
        }
    }
    return Ok(());
}

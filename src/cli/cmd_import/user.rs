use log::info;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use sqlx::{pool::PoolConnection, MySql, Row};

use crate::{
    core::ResultType,
    entity::{model::StringList, user},
    util::argon2_hash,
};
use anyhow::anyhow;
use futures::TryStreamExt;

pub async fn import_user(
    db: &DatabaseConnection,
    hj2: &mut PoolConnection<MySql>,
) -> ResultType<()> {
    info!("Importing: user");
    let mut rows = sqlx::query("SELECT * FROM user").fetch(hj2);
    let mut to_insert = vec![];
    while let Some(row) = rows.try_next().await? {
        let uname: String = row.try_get("username")?;
        info!("User started: {}", uname);
        let model = user::ActiveModel {
            id: Set(row.try_get("id")?),
            banned: Set(row.try_get("banned")?),
            username: Set(uname.clone()),
            password: Set({
                let text = row.try_get::<String, &str>("password")?;
                if text.starts_with("$") {
                    info!(
                        "Update password to argon2: {}",
                        row.try_get::<String, &str>("username")?
                    );
                    text
                } else {
                    argon2_hash(&text).await?
                }
            }),
            description: Set(row.try_get("description")?),
            email: Set(row.try_get("email")?),
            register_time: Set(row.try_get("register_time")?),
            rating: Set(row.try_get("rating")?),
            permission_group: Set(row.try_get("permission_group")?),
            permissions: Set(StringList(
                serde_json::from_str(row.try_get("permissions")?)
                    .map_err(|e| anyhow!("Invalid permissions value: {}", e))?,
            )),
            force_logout_before: Set(chrono::Local::now().timestamp()),
            phone_number: Set(row.try_get("phone_number")?),
            phone_verified: Set(row.try_get("phone_verified")?),
            last_refreshed_cached_accepted_problems: Set(None),
        };
        to_insert.push((uname, model));
    }
    for (name, item) in to_insert.into_iter() {
        item.insert(db).await?;
        info!("User done: {}", name);
    }
    return Ok(());
}

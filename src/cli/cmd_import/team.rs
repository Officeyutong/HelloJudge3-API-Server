use futures::TryStreamExt;
use log::info;
use sea_orm::{
    ActiveModelTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QuerySelect, Set,
};
use sqlx::{MySqlPool, Row};

use crate::{
    core::ResultType,
    entity::{
        contest, problem, problemset, team, team_contest, team_file, team_member, team_problem,
        team_problemset,
    },
};
use anyhow::anyhow;
pub async fn import_team(db: &DatabaseConnection, hj2: &MySqlPool) -> ResultType<()> {
    info!("Importing: team");
    {
        let mut conn = hj2.acquire().await?;
        let mut rows = sqlx::query("SELECT * FROM team").fetch(&mut conn);
        while let Some(row) = rows.try_next().await? {
            let tid: i32 = row.try_get("id")?;
            let name: String = row.try_get("name")?;
            info!("Import: {}, {}", tid, name);
            team::ActiveModel {
                id: Set(tid),
                name: Set(name.clone()),
                description: Set(row.try_get("description")?),
                owner_id: Set(row.try_get("owner_id")?),
                create_time: Set(row.try_get("create_time")?),
                private: Set(row.try_get("private")?),
                invite_code: Set(row.try_get("invite_code")?),
            }
            .insert(db)
            .await?;
            {
                let team_contests: Vec<i32> =
                    serde_json::from_str(row.try_get("team_contests")?)
                        .map_err(|e| anyhow!("Failed to parse team_contests: {}", e))?;
                for (seq, item) in team_contests.iter().enumerate() {
                    if contest::Entity::find_by_id(*item)
                        .limit(1)
                        .count(db)
                        .await?
                        != 0
                    {
                        team_contest::ActiveModel {
                            team_id: Set(tid),
                            contest_id: Set(*item),
                            sequence: Set(seq as i32),
                        }
                        .insert(db)
                        .await?;
                    } else {
                        info!("Ignore team contest: {}", item);
                    }
                }
            }
            {
                let team_problems: Vec<i32> =
                    serde_json::from_str(row.try_get("team_problems")?)
                        .map_err(|e| anyhow!("Failed to parse team_problems: {}", e))?;
                for (seq, item) in team_problems.iter().enumerate() {
                    if problem::Entity::find_by_id(*item)
                        .limit(1)
                        .count(db)
                        .await?
                        != 0
                    {
                        team_problem::ActiveModel {
                            team_id: Set(tid),
                            problem_id: Set(*item),
                            sequence: Set(seq as i32),
                        }
                        .insert(db)
                        .await?;
                    } else {
                        info!("Ignore team problem: {}", item);
                    }
                }
            }
            {
                let team_problemsets: Vec<i32> =
                    serde_json::from_str(row.try_get("team_problemsets")?)
                        .map_err(|e| anyhow!("Failed to parse team_problemset: {}", e))?;
                for (seq, item) in team_problemsets.iter().enumerate() {
                    if problemset::Entity::find_by_id(*item)
                        .limit(1)
                        .count(db)
                        .await?
                        != 0
                    {
                        team_problemset::ActiveModel {
                            team_id: Set(tid),
                            problemset_id: Set(*item),
                            sequence: Set(seq as i32),
                        }
                        .insert(db)
                        .await?;
                    } else {
                        info!("Ignore team problemset: {}", item);
                    }
                }
            }
        }
    }
    {
        let mut conn = hj2.acquire().await?;
        let mut rows = sqlx::query("SELECT * FROM team_member").fetch(&mut conn);
        while let Some(row) = rows.try_next().await? {
            team_member::ActiveModel {
                team_id: Set(row.try_get("team_id")?),
                uid: Set(row.try_get("uid")?),
                is_admin: Set(row.try_get("is_admin")?),
            }
            .insert(db)
            .await?;
        }
    }
    {
        let mut teamfile_copy = Vec::<String>::new();
        let mut conn = hj2.acquire().await?;
        let mut rows = sqlx::query("SELECT * FROM team_file").fetch(&mut conn);
        while let Some(row) = rows.try_next().await? {
            let fid: String = row.try_get("file_id")?;
            teamfile_copy.push(fid.clone());
            team_file::ActiveModel {
                team_id: Set(row.try_get("team_id")?),
                file_id: Set(fid),
                uid: Set(row.try_get("uid")?),
            }
            .insert(db)
            .await?;
        }
        tokio::fs::write(
            "team-files-import.json",
            serde_json::to_string(&teamfile_copy)?,
        )
        .await?;
    }

    return Ok(());
}

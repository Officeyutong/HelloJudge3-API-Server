use anyhow::anyhow;
use log::{info, warn};
use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QuerySelect, TransactionTrait, Condition,
};
use sea_query::{Expr, SimpleExpr};

use crate::{
    config::Config,
    core::ResultType,
    entity::{cached_accepted_problem, user, model, self},
    RELOCK_TTL,
};

#[inline]
fn refresh_accepted_problem_key(uid: i32) -> String {
    format!("hj3-refresh-accepted-problem-{}", uid)
}

pub async fn ensure_accepted_problems_for_user<T: AsRef<Config>>(
    db: &DatabaseConnection,
    lock: relock::Relock,
    config: T,
    uid: i32,
) -> ResultType<()> {
    let lock_guard = match lock
        .try_lock(refresh_accepted_problem_key(uid), RELOCK_TTL)
        .await
    {
        Ok(l) => l,
        Err(e) => {
            if let relock::Error::CanNotGetLock(r) = e {
                info!(
                    "Refreshing {} in progress, no need to refresh again: {:?}",
                    uid, r
                );
                return Ok(());
            } else {
                return Err(anyhow!("Failed to lock: {}", e));
            }
        }
    };

    let last_refresh = user::Entity::find()
        .select_only()
        .column(user::Column::LastRefreshedCachedAcceptedProblems)
        .filter(user::Column::Id.eq(uid))
        .one(db)
        .await?
        .ok_or(anyhow!("Invalid uid: {}", uid))?
        .last_refreshed_cached_accepted_problems;
    let should_refresh = if let Some(last_refresh_time) = last_refresh {
        let now = chrono::Local::now().naive_local();
        (now - last_refresh_time).num_seconds()
            > config.as_ref().common.accepted_problems_refresh_interval as i64
    } else {
        true
    };
    info!("Refreshing accepted problem cache for {}", uid);
    info!("Should refresh: {}", should_refresh);
    if !should_refresh {
        return Ok(());
    }
    let txn = db.begin().await?;
    {
        use cached_accepted_problem::*;
        Entity::delete_many()
            .filter(cached_accepted_problem::Column::Uid.eq(uid))
            .exec(&txn)
            .await
            .map_err(|e| anyhow!("Failed to delete cached accepted problems! ({})", e))?;
    }
    todo!();
    {
        use entity::problem::*;
        Entity::find().select_only().column(Column::Id).filter(
            Condition::any().add(SimpleExpr::SubQuery(()))
        )
    }
    txn.commit()
        .await
        .map_err(|e| anyhow!("Failed to commit: {}", e))?;
    return Ok(());
}

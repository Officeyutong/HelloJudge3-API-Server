use anyhow::anyhow;
use chrono::NaiveDateTime;
use log::info;
use sea_orm::{
    ColumnTrait, Condition, DatabaseConnection, DeriveColumn, EntityTrait, EnumIter,
    FromQueryResult, IdenStatic, QueryFilter, QuerySelect, Set, TransactionTrait,
};
use sea_query::{Expr, IntoTableRef, Query, SimpleExpr};

use crate::{
    config::Config,
    core::ResultType,
    entity::{self, cached_accepted_problem, contest, submission, user},
    RELOCK_TTL,
};

#[inline]
fn refresh_accepted_problem_key(uid: i32) -> String {
    format!("hj3-refresh-accepted-problem-{}", uid)
}
#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
enum QueryAs {
    RefreshTime,
}
pub async fn ensure_accepted_problems_for_user(
    db: &DatabaseConnection,
    lock: relock::Relock,
    config: &Config,
    uid: i32,
) -> ResultType<()> {
    let _lock_guard = match lock
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
    info!("Start refresh for: {}", uid);

    let last_refresh: Option<NaiveDateTime> = user::Entity::find()
        .select_only()
        .column_as(
            user::Column::LastRefreshedCachedAcceptedProblems,
            QueryAs::RefreshTime,
        )
        .filter(user::Column::Id.eq(uid))
        .into_values::<_, QueryAs>()
        .one(db)
        .await?
        .ok_or(anyhow!("Invalid uid: {}", uid))?;
    let should_refresh = if let Some(last_refresh_time) = last_refresh {
        let now = chrono::Local::now().naive_local();
        (now - last_refresh_time).num_seconds()
            > config.common.accepted_problems_refresh_interval as i64
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
    let problem_ids = {
        use entity::problem::*;
        use submission::Column as sCol;
        #[derive(Debug, FromQueryResult)]
        struct Resp {
            id: i32,
        }
        Entity::find()
            .select_only()
            .column(Column::Id)
            .filter(
                Condition::any()
                    .add(
                        /*
                        (SELECT COUNT(Submission.id) WHERE
                        Submission.problem_id = problem.id
                        AND
                        Submission.status = "accepted"
                        AND
                        Submission.contest_id = -1
                        AND
                        Submission.uid = <UID>
                        LIMIT 1) > 0
                        */
                        SimpleExpr::Binary(
                            Box::new(SimpleExpr::SubQuery(Box::new(
                                sea_query::SubQueryStatement::SelectStatement(
                                    Query::select()
                                        .column(sCol::Id)
                                        .from(submission::Entity.into_table_ref())
                                        .and_where(Expr::col(sCol::ProblemId).binary(
                                            sea_query::BinOper::Equal,
                                            Expr::col(Column::Id),
                                        ))
                                        .and_where(Expr::col(sCol::Status).eq("accepted"))
                                        .and_where(Expr::col(sCol::ContestId).eq(-1i32))
                                        .and_where(Expr::col(sCol::Uid).eq(uid))
                                        .limit(1)
                                        .to_owned(),
                                ),
                            ))),
                            sea_query::BinOper::GreaterThan,
                            Box::new(SimpleExpr::Value(sea_orm::Value::Int(Some(0)))),
                        ),
                    )
                    .add(
                        /*
                        (SELECT COUNT(Submission.id) WHERE
                        Submission.problem_id = problem.id
                        AND
                        Submission.status = "accepted"
                        AND
                        Submission.contest_id IN <CLOSED_CONTEST_SUBQUERY>
                        AND
                        Submission.uid = <UID>
                        LIMIT 1) > 0
                        */
                        SimpleExpr::Binary(
                            Box::new(SimpleExpr::SubQuery(Box::new(
                                sea_query::SubQueryStatement::SelectStatement(
                                    Query::select()
                                        .column(sCol::Id)
                                        .from(submission::Entity.into_table_ref())
                                        .and_where(Expr::col(sCol::ProblemId).binary(
                                            sea_query::BinOper::Equal,
                                            Expr::col(Column::Id),
                                        ))
                                        .and_where(Expr::col(sCol::Status).eq("accepted"))
                                        .and_where(
                                            Expr::col(sCol::ContestId).in_subquery(
                                                Query::select()
                                                    .column(contest::Column::Id)
                                                    .from(contest::Entity.into_table_ref())
                                                    .and_where(
                                                        Expr::col(contest::Column::Closed).eq(true),
                                                    )
                                                    .to_owned(),
                                            ),
                                        )
                                        .and_where(Expr::col(sCol::Uid).eq(uid))
                                        .limit(1)
                                        .to_owned(),
                                ),
                            ))),
                            sea_query::BinOper::GreaterThan,
                            Box::new(SimpleExpr::Value(sea_orm::Value::Int(Some(0)))),
                        ),
                    ),
            )
            .into_model::<Resp>()
            .all(&txn)
            .await
            .map_err(|e| anyhow!("Failed to query: {}", e))?
    };
    // 不这样搞的话，sea-orm会生成INSERT INTO XXX VALUES () 的语句，插入失败
    if !problem_ids.is_empty() {
        use cached_accepted_problem::*;
        Entity::insert_many(problem_ids.into_iter().map(|v| ActiveModel {
            problem_id: Set(v.id),
            uid: Set(uid),
        }))
        .exec(&txn)
        .await
        .map_err(|e| anyhow!("Failed to insert: {}", e))?;
    }
    {
        use user::*;
        Entity::update(ActiveModel {
            id: Set(uid),
            last_refreshed_cached_accepted_problems: Set(Some(chrono::Local::now().naive_local())),
            ..Default::default()
        })
        .exec(&txn)
        .await?;
    }
    txn.commit()
        .await
        .map_err(|e| anyhow!("Failed to commit: {}", e))?;
    return Ok(());
}

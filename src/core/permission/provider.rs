use std::collections::HashSet;

use log::error;
use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait, FromQueryResult, PaginatorTrait, QueryFilter,
    QuerySelect,
};

use crate::{
    core::ResultType,
    entity::{challenge_problemset, challenge_record, model::StringList, team, team_member},
};

use super::PermissionSet;
use anyhow::anyhow;
#[async_trait::async_trait]
pub trait PermissionProvider: Sync + Send {
    async fn get_permission(
        &self,
        db: &DatabaseConnection,
        uid: Option<i32>,
        arg: Option<String>,
    ) -> ResultType<PermissionSet>;
}

pub struct ChallengeFinishProvider;
#[async_trait::async_trait]
impl PermissionProvider for ChallengeFinishProvider {
    async fn get_permission(
        &self,
        _db: &DatabaseConnection,
        _uid: Option<i32>,
        arg: Option<String>,
    ) -> ResultType<PermissionSet> {
        return Ok(HashSet::from([format!(
            "challenge.finish.{}.all",
            arg.ok_or(anyhow!("Expected argument!"))?
        )]));
    }
}

pub struct AllChallengeAccessProvider;

#[async_trait::async_trait]
impl PermissionProvider for AllChallengeAccessProvider {
    async fn get_permission(
        &self,
        db: &DatabaseConnection,
        uid: Option<i32>,
        _arg: Option<String>,
    ) -> ResultType<PermissionSet> {
        let uid = uid.ok_or(anyhow!("Login users only!"))?;
        #[derive(Debug, FromQueryResult)]
        struct Resp {
            challenge_id: i32,
            problemset_id: i32,
            finished: bool,
        }
        let access = challenge_record::Entity::find()
            .select_only()
            .column(challenge_record::Column::ChallengeId)
            .column(challenge_record::Column::ProblemsetId)
            .column(challenge_record::Column::Finished)
            .filter(challenge_record::Column::Uid.eq(uid))
            .into_model::<Resp>()
            .all(db)
            .await?;
        let mut ret: PermissionSet = HashSet::default();
        let mut all_challenges: HashSet<i32> = HashSet::default();
        let mut exist_unfinished: HashSet<i32> = HashSet::default();
        for Resp {
            challenge_id,
            problemset_id,
            finished,
        } in access.into_iter()
        {
            all_challenges.insert(challenge_id);
            if finished {
                ret.insert(format!(
                    "challenge.finish.{}.{}",
                    challenge_id, problemset_id
                ));
            } else {
                exist_unfinished.insert(challenge_id);
            }
        }
        for item in all_challenges.into_iter() {
            ret.insert(format!("[provider:challenge-access.{}]", item));
            if !exist_unfinished.contains(&item) {
                ret.insert(format!("challenge.finish.{}.all", item));
            }
        }
        return Ok(ret);
    }
}

pub struct ChallengeAccessProvider;
#[async_trait::async_trait]
impl PermissionProvider for ChallengeAccessProvider {
    async fn get_permission(
        &self,
        db: &DatabaseConnection,
        _uid: Option<i32>,
        arg: Option<String>,
    ) -> ResultType<PermissionSet> {
        let cid = arg
            .map(|v| i32::from_str_radix(&v, 10))
            .transpose()?
            .ok_or(anyhow!("Expect arg: challenge_id"))?;
        use challenge_problemset::*;
        #[derive(Debug, FromQueryResult)]
        struct Resp {
            // challenge_id:i32,
            problemset_id: i32,
            // finished:bool
        }
        let problem_sets = Entity::find()
            .select_only()
            .column(Column::ProblemsetId)
            .filter(Column::ChallengeId.eq(cid))
            .into_model::<Resp>()
            .all(db)
            .await?;
        let mut result = PermissionSet::default();
        if !problem_sets.is_empty() {
            for Resp { problemset_id } in problem_sets.into_iter() {
                result.insert(format!("[provider:problemset.{}]", problemset_id));
            }
            result.insert(format!("challenge.access.{}", cid));
        }
        return Ok(result);
    }
}

pub struct AllTeamsPermissionsProvider;
#[async_trait::async_trait]
impl PermissionProvider for AllTeamsPermissionsProvider {
    async fn get_permission(
        &self,
        db: &DatabaseConnection,
        uid: Option<i32>,
        _arg: Option<String>,
    ) -> ResultType<PermissionSet> {
        use crate::entity::team_member::*;
        #[derive(Debug, FromQueryResult)]
        struct Resp {
            // challenge_id:i32,
            // problemset_id:i32,
            // finished:bool
            team_id: i32,
        }
        let teams = Entity::find()
            .select_only()
            .column(Column::TeamId)
            .filter(Column::Uid.eq(uid.ok_or(anyhow!("Login users only!"))?))
            .into_model::<Resp>()
            .all(db)
            .await?;
        return Ok(HashSet::from_iter(
            teams
                .into_iter()
                .map(|Resp { team_id }| format!("[provider:team.{}]", team_id)),
        ));
    }
}

pub struct ContestPermissionProvider;
#[async_trait::async_trait]
impl PermissionProvider for ContestPermissionProvider {
    async fn get_permission(
        &self,
        _db: &DatabaseConnection,
        _uid: Option<i32>,
        arg: Option<String>,
    ) -> ResultType<PermissionSet> {
        let arg = arg.ok_or(anyhow!("Expect arg: contest_id"))?;
        return Ok(HashSet::from([format!("contest.use.{}", arg)]));
    }
}

pub struct TeamPermissionsProvider;
#[async_trait::async_trait]
impl PermissionProvider for TeamPermissionsProvider {
    async fn get_permission(
        &self,
        db: &DatabaseConnection,
        uid: Option<i32>,
        arg: Option<String>,
    ) -> ResultType<PermissionSet> {
        let uid = uid.ok_or(anyhow!("Login required"))?;
        let team_id = arg
            .map(|v| i32::from_str_radix(&v, 10))
            .transpose()?
            .ok_or(anyhow!("Expected argument: team_id"))?;

        let team_exists = team::Entity::find_by_id(team_id).limit(1).count(db).await? == 1;
        if !team_exists {
            error!("Invalid team: {}", team_id);
            return Ok(PermissionSet::default());
        }
        let mut result = HashSet::from([format!("team.use.{}", team_id)]);
        let joined = team_member::Entity::find()
            .filter(
                team_member::Column::TeamId
                    .eq(team_id)
                    .and(team_member::Column::Uid.eq(uid)),
            )
            .limit(1)
            .count(db)
            .await?
            != 0;
        if joined {
            {
                use crate::entity::team_problem::*;
                #[derive(Debug, FromQueryResult)]
                struct Resp {
                    // challenge_id:i32,
                    // problemset_id:i32,
                    // finished:bool
                    problem_id: i32,
                }
                for Resp { problem_id } in Entity::find()
                    .select_only()
                    .column(Column::ProblemId)
                    .filter(Column::TeamId.eq(team_id))
                    .into_model::<Resp>()
                    .all(db)
                    .await?
                {
                    result.insert(format!("problem.use.{}", problem_id));
                }
            }
            {
                use crate::entity::team_problemset::*;
                #[derive(Debug, FromQueryResult)]
                struct Resp {
                    // challenge_id:i32,
                    problemset_id: i32,
                    // finished:bool
                }
                for Resp { problemset_id } in Entity::find()
                    .select_only()
                    .column(Column::ProblemsetId)
                    .filter(Column::TeamId.eq(team_id))
                    .into_model::<Resp>()
                    .all(db)
                    .await?
                {
                    result.insert(format!("[provider:problemset.{}]", problemset_id));
                }
            }
            {
                use crate::entity::team_contest::*;
                #[derive(Debug, FromQueryResult)]
                struct Resp {
                    // challenge_id:i32,
                    // problemset_id:i32,
                    // finished:bool
                    contest_id: i32,
                }
                for Resp { contest_id, .. } in Entity::find()
                    .select_only()
                    .column(Column::ContestId)
                    .filter(Column::TeamId.eq(team_id))
                    .into_model::<Resp>()
                    .all(db)
                    .await?
                {
                    result.insert(format!("[provider:contest.{}]", contest_id));
                }
            }
        }
        return Ok(result);
    }
}

pub struct ProblemsetPermissionProvider;
#[async_trait::async_trait]
impl PermissionProvider for ProblemsetPermissionProvider {
    async fn get_permission(
        &self,
        db: &DatabaseConnection,
        _uid: Option<i32>,
        arg: Option<String>,
    ) -> ResultType<PermissionSet> {
        let set_id = arg
            .map(|v| i32::from_str_radix(&v, 10))
            .transpose()?
            .ok_or(anyhow!("Expect problemset id!"))?;
        let mut result = HashSet::from([format!("problemset.use.{}", set_id)]);
        use crate::entity::problemset_problem::*;
        #[derive(Debug, FromQueryResult)]
        struct Resp {
            // challenge_id:i32,
            // problemset_id:i32,
            // finished:bool
            problem_id: i32,
        }
        for Resp { problem_id, .. } in Entity::find()
            .select_only()
            .column(Column::ProblemId)
            .filter(Column::ProblemsetId.eq(set_id))
            .into_model::<Resp>()
            .all(db)
            .await?
        {
            result.insert(format!("problem.use.{}", problem_id));
        }
        return Ok(result);
    }
}

pub struct PermissionPackPermissionsProvider;
#[async_trait::async_trait]
impl PermissionProvider for PermissionPackPermissionsProvider {
    async fn get_permission(
        &self,
        db: &DatabaseConnection,
        _uid: Option<i32>,
        arg: Option<String>,
    ) -> ResultType<PermissionSet> {
        let pid = arg
            .map(|v| i32::from_str_radix(&v, 10))
            .transpose()?
            .ok_or(anyhow!("Expect argument: permission_pack id"))?;
        let mut result = HashSet::<String>::default();
        use crate::entity::permission_pack::*;
        #[derive(Debug, FromQueryResult)]
        struct Resp {
            // challenge_id:i32,
            // problemset_id:i32,
            // finished:bool
            permissions: StringList,
        }
        let perm_pack = Entity::find_by_id(pid)
            .select_only()
            .column(Column::Permissions)
            .into_model::<Resp>()
            .one(db)
            .await?;
        if let Some(v) = perm_pack {
            result.insert(format!("permissionpack.claimed.{}", pid));
            result.extend(v.permissions.0.into_iter());
        }
        return Ok(result);
    }
}

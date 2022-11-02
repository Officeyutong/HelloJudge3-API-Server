use std::{collections::HashSet, sync::Arc};

use crate::{
    core::{sql_func::JsonArrayAppend, ResultType},
    entity::{model::StringList, permission_group, user},
};

use super::{PermissionManager, PermissionSet};
use anyhow::anyhow;
use log::debug;
use sea_orm::{
    sea_query::{
        Expr, Function, IntoColumnRef, IntoTableRef, MysqlQueryBuilder, Query, SimpleExpr,
    },
    EntityTrait, FromQueryResult, QuerySelect, Updater, Value,
};
impl PermissionManager {
    pub async fn add_permission(&self, uid: Option<i32>, perm: &str) -> ResultType<()> {
        if let Some(uid) = uid {
            let update_stmt = Query::update()
                .table(user::Entity.into_table_ref())
                .and_where(Expr::col(user::Column::Id).eq(uid))
                .value_expr(
                    user::Column::Permissions,
                    SimpleExpr::FunctionCall(
                        Function::Custom(Arc::new(JsonArrayAppend)),
                        vec![
                            SimpleExpr::Column(user::Column::Permissions.into_column_ref()),
                            SimpleExpr::Value(Value::String(Some(Box::new("$".to_string())))),
                            SimpleExpr::Value(Value::String(Some(Box::new(perm.to_string())))),
                        ],
                    ),
                )
                .to_owned();

            debug!(
                "Update permission for {},\n{}",
                uid,
                update_stmt.to_string(MysqlQueryBuilder)
            );
            Updater::new(update_stmt)
                .exec(&*self.db)
                .await
                .map_err(|e| anyhow!("Failed to update permissions: {}", e))?;
            self.refresh_user(Some(uid)).await?;
            return Ok(());
        } else {
            return Ok(());
        }
    }
    pub async fn get_permission_from_database(
        &self,
        uid: Option<i32>,
    ) -> ResultType<PermissionSet> {
        if let Some(uid) = uid {
            #[derive(Debug, FromQueryResult)]
            struct Resp {
                permissions: StringList,
                permission_group: String,
            }
            let mut perms: PermissionSet = HashSet::default();
            let user = user::Entity::find_by_id(uid)
                .select_only()
                .column(user::Column::Permissions)
                .column(user::Column::PermissionGroup)
                .into_model::<Resp>()
                .one(&*self.db)
                .await?
                .ok_or(anyhow!("Invalid user: {}", uid))?;
            let mut perm_group = user.permission_group;
            perms.extend(user.permissions.0.into_iter());
            while !perm_group.is_empty() {
                #[derive(Debug, FromQueryResult)]
                struct Resp {
                    permissions: StringList,
                    inherit: Option<String>,
                }
                let curr = permission_group::Entity::find_by_id(perm_group.clone())
                    .select_only()
                    .column(permission_group::Column::Inherit)
                    .column(permission_group::Column::Permissions)
                    .into_model::<Resp>()
                    .one(&*self.db)
                    .await?
                    .ok_or(anyhow!(
                        "Invalid permission group on the extending chain: {}",
                        perm_group
                    ))?;
                perms.extend(curr.permissions.0.into_iter());
                if let Some(v) = curr.inherit {
                    perm_group = v;
                } else {
                    break;
                }
            }
            debug!("User permissions {}: {:?}", uid, perms);
            return Ok(perms);
        } else {
            return Ok(self.default_permissions.clone());
        }
    }
}

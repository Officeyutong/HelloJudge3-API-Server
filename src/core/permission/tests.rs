use std::{collections::HashSet, sync::Arc};

use sea_orm::MockDatabase;

use crate::entity::{model::StringList, permission_group, user};

use super::PermissionManager;

#[tokio::test]
pub async fn test_permission_operator() {
    let user = user::Model {
        id: 233,
        permissions: StringList(vec!["user-perm1".into(), "user-perm2".into()]),
        banned: false,
        username: "aaa".into(),
        password: "".into(),
        description: "".into(),
        email: "".into(),
        register_time: chrono::Local::now().naive_local(),
        rating: 1111,
        permission_group: "group1".into(),
        force_logout_before: 0,
        phone_number: None,
        phone_verified: false,
        last_refreshed_cached_accepted_problems: None,
    };
    let group1 = permission_group::Model {
        id: "group1".into(),
        name: "".into(),
        permissions: StringList(vec!["group1-perm1".into(), "group1-perm2".into()]),
        inherit: Some("group2".into()),
    };
    let group2 = permission_group::Model {
        id: "group2".into(),
        name: "".into(),
        permissions: StringList(vec!["group2-perm1".into(), "group2-perm2".into()]),
        inherit: None,
    };
    let db = MockDatabase::new(sea_orm::DatabaseBackend::MySql)
        .append_query_results(vec![vec![user]])
        .append_query_results(vec![vec![group1], vec![group2]])
        .into_connection();
    let perm_manager = PermissionManager::new(
        Arc::new(db),
        redis::Client::open("redis://127.0.0.1").unwrap(),
        HashSet::from(["default-permission".into()]),
    );
    let val = perm_manager
        .get_permission_from_database(Some(233))
        .await
        .unwrap();
    assert_eq!(
        val,
        HashSet::from([
            "user-perm1".into(),
            "user-perm2".into(),
            "group1-perm1".into(),
            "group1-perm2".into(),
            "group2-perm1".into(),
            "group2-perm2".into()
        ])
    );
}

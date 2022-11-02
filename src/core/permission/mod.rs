use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use self::provider::PermissionProvider;
use super::{redis_key_perm, ResultType};
use anyhow::anyhow;
use log::error;
use redis::AsyncCommands;
use regex::Regex;
use sea_orm::DatabaseConnection;
pub mod operator;
pub mod provider;
pub type PermissionSet = HashSet<String>;
#[cfg(test)]
mod tests;

lazy_static::lazy_static! {
    static ref PROVIDER_REF_EXPR:Regex = Regex::new(r#"\[provider:(?P<name>[a-zA-Z0-9\-]+)(\.(?P<arg>.+))?\]"#).unwrap();
}
pub struct PermissionManager {
    db: Arc<DatabaseConnection>,
    // lock: Relock,
    providers: HashMap<String, Arc<dyn PermissionProvider>>,
    default_permissions: PermissionSet,
    redis: redis::Client,
}

impl PermissionManager {
    pub fn new(
        db: Arc<DatabaseConnection>,
        redis_client: redis::Client,
        default_permissions: PermissionSet,
    ) -> Self {
        Self {
            db,
            default_permissions,
            // lock,
            redis: redis_client,
            providers: HashMap::default(),
        }
    }
    pub fn add_provider(&mut self, name: &str, provider: Arc<dyn PermissionProvider>) {
        self.providers.insert(name.to_string(), provider);
    }
    pub async fn refresh_user(&self, uid: Option<i32>) -> ResultType<()> {
        if let Some(uid) = uid {
            let mut conn = self.redis.get_async_connection().await?;
            conn.del(redis_key_perm(uid)).await?;
        }
        Ok(())
    }
    pub async fn has_permission(&self, uid: Option<i32>, perm: &str) -> ResultType<bool> {
        if let Some(uid) = uid {
            let mut conn = self.redis.get_async_connection().await?;
            let set_name = redis_key_perm(uid);
            if !conn.exists(&set_name).await? {
                self.load_into_cache(uid).await?;
            }
            if conn.sismember(&set_name, format!("-{}", perm)).await? {
                return Ok(false);
            }
            if conn.sismember(&set_name, "*").await? || conn.sismember(&set_name, perm).await? {
                return Ok(true);
            }
            let split = perm.split(".").collect::<Vec<&str>>();
            for i in 0..split.len() {
                // 前i个连起来
                let curr_str = format!("{}.*", split[..i].join("."));
                if conn.sismember(&set_name, curr_str).await? {
                    return Ok(true);
                }
            }
            return Ok(false);
        } else {
            Ok(self.default_permissions.contains(perm))
        }
    }
    pub async fn get_all_permissions(&self, uid: Option<i32>) -> ResultType<Vec<String>> {
        if let Some(uid) = uid {
            let set_name = redis_key_perm(uid);
            let mut conn = self.redis.get_async_connection().await?;
            if !conn.exists(&set_name).await? {
                self.load_into_cache(uid).await?;
            }
            let strs: Vec<String> = conn.smembers(&set_name).await?;
            return Ok(strs);
        } else {
            return Ok(Vec::from_iter(self.default_permissions.clone().into_iter()));
        }
    }
    async fn load_into_cache(&self, uid: i32) -> ResultType<()> {
        let mut conn = self.redis.get_async_connection().await?;
        let set_name = redis_key_perm(uid);
        let permission = self
            .rec_parse_permission(
                &self.get_permission_from_database(Some(uid)).await?,
                uid,
                None,
            )
            .await?;
        if permission.is_empty() {
            return Ok(());
        }
        conn.del(&set_name).await?;
        conn.sadd(&set_name, permission.into_iter().collect::<Vec<String>>())
            .await?;
        conn.expire(&set_name, 60).await?;
        return Ok(());
    }
    #[async_recursion::async_recursion]
    async fn rec_parse_permission(
        &self,
        perm: &PermissionSet,
        uid: i32,
        log: Option<PermissionSet>,
    ) -> ResultType<PermissionSet> {
        let mut result: PermissionSet = HashSet::default();
        let mut providers = Vec::<String>::new();
        for x in perm.iter() {
            if PROVIDER_REF_EXPR.is_match(x) {
                providers.push(x.into())
            } else {
                result.insert(x.to_string());
            }
        }
        let local_log = log.unwrap_or_default();
        for x in providers.into_iter() {
            if local_log.contains(&x) {
                error!(
                    "Recursive provider reference when \
                parsing uid `{}`, provider: `{}`, log: `{:?}`",
                    uid, x, local_log
                );
                return Ok(result);
            } else {
                result.extend(
                    self.rec_parse_permission(&self.parse_permission(uid, &x).await?, uid, {
                        let mut local_log = local_log.clone();
                        local_log.insert(x.clone());
                        Some(local_log)
                    })
                    .await?,
                );
            }
        }
        return Ok(result);
    }
    async fn parse_permission(&self, uid: i32, text: &str) -> ResultType<PermissionSet> {
        if let Some(my_match) = PROVIDER_REF_EXPR.captures(text) {
            let name = my_match.name("name").unwrap().as_str();
            let arg = my_match.name("arg").map(|v| v.as_str().into());
            if let Some(provider) = self.providers.get(name).cloned() {
                return Ok(provider
                    .get_permission(&self.db, Some(uid), arg)
                    .await
                    .map_err(|e| {
                        anyhow!("Error occurred when invoking permission provider: {}", e)
                    })?);
            } else {
                error!(
                    "Invalid provider: {} for uid {}, empty permissions returned.",
                    name, uid
                );
                return Ok(HashSet::default());
            }
        } else {
            return Ok(HashSet::from([text.into()]));
        }
    }
}

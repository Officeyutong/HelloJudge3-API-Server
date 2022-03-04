use serde::{Deserialize, Serialize};

use self::{
    auth::AuthConfig, common::CommonConfig, display::DisplayConfig, judge::JudgeConfig,
    judge_status::JudgeStatusConfig, phoneauth::PhoneauthConfig, redis::RedisConfig,
};

pub mod auth;
pub mod common;
pub mod display;
pub mod judge;
pub mod judge_status;
pub mod phoneauth;
pub mod redis;

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Config {
    pub auth: AuthConfig,
    pub common: CommonConfig,
    pub phoneauth: PhoneauthConfig,
    pub redis: RedisConfig,
    pub display: DisplayConfig,
    pub judge: JudgeConfig,
    pub judge_status: JudgeStatusConfig,
}

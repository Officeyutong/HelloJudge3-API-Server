use actix_session::Session;
use actix_web::error;
use anyhow::anyhow;
use chrono::{Local, Utc};

use crate::util::log_ise;

use super::{ActixResult, ResultType, SESSION_UID};
pub struct MyUserSession {
    pub uid: i32,
    pub login_time: chrono::DateTime<Local>,
}
impl MyUserSession {
    pub fn parse(session: &Session) -> ResultType<Self> {
        let uid = session
            .get::<i32>("uid")
            .map_err(|e| anyhow!("Failed to get cookie for uid: {}", e))?
            .ok_or_else(|| anyhow!("你尚未登陆!"))?;
        let login_time = session
            .get::<i64>("login_time")
            .map_err(|e| anyhow!("Failed to get cookie for login_time: {}", e))?
            // .map_err(|e| e.into())?
            .ok_or_else(|| anyhow!("你尚未登陆!"))?;
        let parsed_time = chrono::DateTime::<Local>::from(chrono::DateTime::<Utc>::from_utc(
            chrono::NaiveDateTime::from_timestamp(login_time, 0),
            Utc,
        ));
        return Ok(Self {
            uid,
            login_time: parsed_time,
        });
    }
}

pub trait ParsedSessionState {
    fn to_state(&self) -> ResultType<MyUserSession>;
    fn uid(&self) -> ActixResult<i32>;
}

impl ParsedSessionState for Session {
    fn to_state(&self) -> ResultType<MyUserSession> {
        MyUserSession::parse(self)
    }
    fn uid(&self) -> ActixResult<i32> {
        let val = self
            .get::<i32>(SESSION_UID)
            .map_err(|e| anyhow!("Failed to get uid: {}", e))
            .map_err(log_ise)?;
        return Ok(val.ok_or(error::ErrorInternalServerError("请先登录！"))?);
    }
}

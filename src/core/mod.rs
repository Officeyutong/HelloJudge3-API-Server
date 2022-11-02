use std::collections::HashMap;

use actix_web::{body::BoxBody, error, HttpResponse, Responder};
use anyhow::anyhow;
use log::error;
use serde::Serialize;
pub type ResultType<T> = anyhow::Result<T>;
pub type ActixResult<T> = actix_web::Result<T>;
pub mod file;
pub mod model;
pub mod permission;
pub mod result;
pub mod session;
pub mod sql_func;
pub mod state;
pub mod user;
pub const SYSTEM_NOTIFICATION_USERID: i32 = 1;
pub const SESSION_UID: &str = "uid";
pub const SESSION_LOGIN_TIME: &str = "login_time";
#[inline]
pub fn redis_key_perm(uid: i32) -> String {
    format!("hj3-perm-{}", uid)
}
#[inline]
pub fn redis_key_email_auth(token: &str) -> String {
    format!("hj3-email-auth-{}", token)
}
#[inline]
pub fn redis_key_reset_password(token: &str) -> String {
    format!("hj3-email-reset-password-{}", token)
}

// #[derive(Serialize)]
pub struct MySimpleRawResponse<T: Serialize> {
    message: Option<String>,
    code: i32,
    data: Option<T>,
}

pub type MySimpleResponse = MySimpleRawResponse<serde_json::Value>;

// pub struct MySimpleResponse {
//     message: Option<String>,
//     code: i32,
//     data: Option<serde_json::Value>,
// }

pub fn msg_err(msg: &str) -> MySimpleResponse {
    MySimpleResponse {
        code: -1,
        data: None,
        message: Some(msg.into()),
    }
}
pub fn msg_err_wrp(msg: &str) -> ActixResult<MySimpleResponse> {
    Ok(MySimpleResponse {
        code: -1,
        data: None,
        message: Some(msg.into()),
    })
}
pub fn ok_wrp() -> ActixResult<MySimpleResponse> {
    Ok(MySimpleResponse {
        code: 0,
        data: None,
        message: None,
    })
}
pub fn msg_ok_wrp(msg: &str) -> ActixResult<MySimpleResponse> {
    Ok(MySimpleResponse {
        code: 0,
        data: None,
        message: Some(msg.into()),
    })
}
pub fn ok_data_wrp<T>(v: &T) -> ActixResult<MySimpleResponse>
where
    T: Serialize,
{
    Ok(MySimpleResponse {
        code: 0,
        message: None,
        data: Some(
            serde_json::to_value(&v)
                .map_err(|e| anyhow!("Failed to deserialize: {}", e))
                .map_err(error::ErrorInternalServerError)?,
        ),
    })
}
pub fn ok_data_msg_wrp<T>(v: &T, msg: &str) -> ActixResult<MySimpleResponse>
where
    T: Serialize,
{
    Ok(MySimpleResponse {
        code: 0,
        message: Some(msg.to_string()),
        data: Some(
            serde_json::to_value(&v)
                .map_err(|e| anyhow!("Failed to deserialize: {}", e))
                .map_err(error::ErrorInternalServerError)?,
        ),
    })
}
// impl<T, E> FromResidual<Result<T, E>> for MySimpleResponse
// where
//     E: ToString,
// {
//     fn from_residual(residual: Result<T, E>) -> Self {
//         match residual {
//             Ok(_) => Self {
//                 code: 0,
//                 data: None,
//                 message: None,
//             },
//             Err(e) => Self {
//                 code: -1,
//                 data: None,
//                 message: Some(e.to_string()),
//             },
//         }
//     }
// }

impl<T: Serialize> From<T> for MySimpleResponse {
    fn from(v: T) -> Self {
        Self {
            code: 0,
            data: Some(serde_json::to_value(v).unwrap()),
            message: None,
        }
    }
}

impl<T: Serialize> Responder for MySimpleRawResponse<T> {
    type Body = BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        use serde_json::json;
        use serde_json::Value;
        let mut my_map = HashMap::<String, Value>::default();
        my_map.insert("code".into(), json!(self.code));
        if let Some(v) = self.data {
            my_map.insert(
                "data".into(),
                match serde_json::to_value(v) {
                    Ok(v2) => v2,
                    Err(e) => {
                        error!("Failed to serialize json: {}", e);
                        json!({})
                    }
                },
            );
        }
        if let Some(v) = self.message {
            my_map.insert("message".into(), json!(v));
        }
        HttpResponse::Ok().json(my_map)
    }
}

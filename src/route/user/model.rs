use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct EmailAuthInfo {
    pub username: String,
    pub email: String,
    pub password: String,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct ResetPasswordInfo {
    pub uid: i32,
}

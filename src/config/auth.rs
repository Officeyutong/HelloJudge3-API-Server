use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct AuthConfig {
    pub smtp_using_ssl: bool,
    pub smtp_server: String,
    pub smtp_port: u16,
    pub smtp_user: String,
    pub smtp_password: String,
    pub email_sender: String,
    pub reset_password_email: String,
    pub require_register_auth: bool,
    pub register_auth_email: String,
    pub change_email_auth_email: String,
    pub auth_password: String,
    pub auth_token: String,
    pub reset_password_expire_seconds: u32,
    pub register_email_auth_expire_seconds: u32,
    pub change_email_expire_seconds: u32,
    pub use_phone_when_register_ans_resetpasswd: bool,
    pub recaptcha_site_key: String,
    pub recaptcha_secret: String,
}
impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            smtp_using_ssl: false,
            smtp_server: "smtp.qwq.com".to_string(),
            smtp_port: 25,
            smtp_user: "qwqqwq".to_owned(),
            smtp_password: "password".to_string(),
            email_sender: "qwq@qwq.com".to_string(),
            reset_password_email: "请前往 http://[此处替换为网站地址]/reset_password/{reset_token} 重置密码。此链接仅有效一次。".to_string(),
            require_register_auth: false,
            register_auth_email: "请前往 http://[此处替换为网站地址]/auth_email/{auth_token} 激活账号。此链接仅有效一次。".to_string(),
            change_email_auth_email:"请前往 http://[此处替换为网站地址]/api/change_email/{change_token} 更改邮箱。".to_string(),
            auth_password: "11223344556677889900aabbccddeeff".to_string(),
            auth_token: "d2c25808-28b2-11ea-acf5-9cda3efd56be".to_string(),
            reset_password_expire_seconds: 30*60,
            register_email_auth_expire_seconds: 30*60,
            change_email_expire_seconds: 30*60,
            use_phone_when_register_ans_resetpasswd: false,
            recaptcha_site_key: "recaptcha-site-key".to_string(),
            recaptcha_secret: "recaptcha-secret".to_string(),
        }
    }
}

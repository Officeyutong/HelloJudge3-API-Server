use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PhoneauthConfig {
    pub code_validity: i64,
    pub min_send_gap: i64,
    pub aliyun_access_key_id: String,
    pub aliyun_access_secret: String,
    pub aliyun_region: String,
    pub aliyun_sign_name: String,
    pub aliyun_template_code: String,
    pub aliyun_captcha_app_key: String,
}

impl Default for PhoneauthConfig {
    fn default() -> Self {
        Self {
            code_validity: 300,
            min_send_gap: 60,
            aliyun_access_key_id: "accessKeyId".into(),
            aliyun_access_secret: "accessSecret".into(),
            aliyun_region: "cn-hangzhou".into(),
            aliyun_sign_name: "SignName".into(),
            aliyun_template_code: "TemplateCode".into(),
            aliyun_captcha_app_key: "CaptchaAppKey".into(),
        }
    }
}

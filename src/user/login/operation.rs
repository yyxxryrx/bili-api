use serde::{Deserialize, Serialize};

use crate::APIResponse;

pub mod qrcode;
pub mod sms;

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginCaptchaData {
    /// 极验captcha数据
    pub geetest: LoginCaptchaGeetest,
    /// 作用尚不明确
    pub tencent: serde_json::Value,
    /// 登录 API token (与 captcha 无关，与登录接口有关)
    pub token: String,
    /// 验证方式
    #[serde(rename = "type", with = "serde_login_captcha_type")]
    pub captcha_type: LoginCaptchaType,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginCaptchaGeetest {
    pub gt: String,
    pub challenge: String,
}

#[derive(Debug, Clone)]
pub enum LoginCaptchaType {
    Geetest,
    Other(String),
}

impl From<&str> for LoginCaptchaType {
    fn from(value: &str) -> Self {
        if value.to_lowercase() == "geetest" {
            Self::Geetest
        } else {
            Self::Other(value.to_string())
        }
    }
}

impl From<String> for LoginCaptchaType {
    fn from(value: String) -> Self {
        if value.to_lowercase() == "geetest" {
            Self::Geetest
        } else {
            Self::Other(value.to_string())
        }
    }
}

impl From<&LoginCaptchaType> for String {
    fn from(value: &LoginCaptchaType) -> Self {
        match value {
            LoginCaptchaType::Geetest => "geetest".to_string(),
            LoginCaptchaType::Other(t) => t.clone(),
        }
    }
}

mod serde_login_captcha_type {
    use super::LoginCaptchaType;
    use serde::{Deserialize, Deserializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<LoginCaptchaType, D::Error>
    where
        D: Deserializer<'de>,
    {
        let t = String::deserialize(deserializer)?;
        Ok(t.into())
    }

    pub fn serialize<S>(captcha_type: &LoginCaptchaType, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let t = String::from(captcha_type);
        serializer.serialize_str(&t)
    }
}

pub async fn get_captcha(client: reqwest::Client) -> crate::error::APIResult<LoginCaptchaData> {
    let response = client
        .get("https://passport.bilibili.com/x/passport-login/captcha?source=main_web")
        .send()
        .await?;
    let json = response.text().await?;
    let resp: APIResponse<LoginCaptchaData> = serde_json::from_str(&json)?;
    resp.into_result()
}

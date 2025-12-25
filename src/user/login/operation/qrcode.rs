use crate::{APIResponse, error::APIResult};
use serde::{Deserialize, Serialize};
use std::{cmp::PartialEq, collections::HashMap};

/// 请求登录二维码返回的数据
#[derive(Debug, Deserialize, Serialize)]
struct LoginQRGenData {
    url: String,
    qrcode_key: String,
}

/// 登录二维码状态
#[derive(PartialEq, Debug)]
pub enum LoginQRStatue {
    NoScan,
    TimeoutOrFailure,
    Scanned,
    Success,
}

impl Default for LoginQRStatue {
    fn default() -> Self {
        Self::NoScan
    }
}

impl LoginQRStatue {
    /// 从状态码创建状态
    pub fn from_code(code: i32) -> Option<Self> {
        match code {
            0 => Some(Self::Success),
            86038 => Some(Self::TimeoutOrFailure),
            86090 => Some(Self::Scanned),
            86101 => Some(Self::NoScan),
            _ => None,
        }
    }

    /// 是否正在等待
    pub fn is_waiting(&self) -> bool {
        match self {
            Self::Success => false,
            Self::TimeoutOrFailure => false,
            Self::Scanned => true,
            Self::NoScan => true,
        }
    }

    /// 是否登录成功
    pub fn is_success(&self) -> bool {
        match self {
            Self::Success => true,
            _ => false,
        }
    }
}

/// 轮询二维码状态返回数据
#[derive(Debug, Deserialize, Serialize)]
struct LoginQRRefreshData {
    url: String,
    refresh_token: String,
    timestamp: u64,
    code: i32,
    message: String,
}

/// 工具类，帮你管理二维码生命周期和拿取cookie
pub struct LoginQR {
    key: String,
    pub statue: LoginQRStatue,
    pub qrcode_url: String,
}

impl LoginQR {
    pub fn new(key: String, qrcode_url: String) -> Self {
        Self {
            key,
            qrcode_url,
            statue: Default::default(),
        }
    }

    /// 轮询，登录成功返回Cookie键值对，还没登上返回None
    pub async fn refresh(
        &mut self,
        client: &reqwest::Client,
    ) -> APIResult<Option<HashMap<String, String>>> {
        if self.statue == LoginQRStatue::Success {
            return Ok(None);
        }
        let response = client
            .get("https://passport.bilibili.com/x/passport-login/web/qrcode/poll")
            .query(&[("qrcode_key", &self.key)])
            .send()
            .await?;
        let cookies = response.cookies().fold(HashMap::new(), |mut acc, cookie| {
            acc.insert(cookie.name().to_string(), cookie.value().to_string());
            acc
        });
        let json = &response.text().await?;
        let resp: APIResponse<LoginQRRefreshData> = serde_json::from_str(json)?;
        let resp = resp.into_result()?;
        let Some(statue) = LoginQRStatue::from_code(resp.code) else {
            return Err(crate::error::Error::Unknown("Unknown code".to_string()));
        };
        self.statue = statue;
        Ok(if self.statue == LoginQRStatue::Success {
            Some(cookies)
        } else {
            None
        })
    }

    #[cfg(feature = "summon_qrcode")]
    pub fn get_qrcode(&self) -> qrcode::QrResult<image::DynamicImage> {
        let code = qrcode::QrCode::new(self.qrcode_url.as_bytes())?;
        Ok(image::DynamicImage::from(
            code.render::<image::Luma<u8>>().build(),
        ))
    }
}

/// 申请登录二维码
pub async fn get_qrcode(client: &reqwest::Client) -> APIResult<LoginQR> {
    let response = client
        .get("https://passport.bilibili.com/x/passport-login/web/qrcode/generate")
        .send()
        .await?;
    let resp: APIResponse<LoginQRGenData> = serde_json::from_str(&response.text().await?)?;
    let resp = resp.into_result()?;
    Ok(LoginQR::new(resp.qrcode_key, resp.url))
}

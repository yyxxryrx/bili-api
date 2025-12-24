use crate::error::APIResult;
use serde::Deserialize;
use std::{cmp::PartialEq, collections::HashMap};

#[derive(Deserialize)]
struct LoginQRGenRespData {
    url: String,
    qrcode_key: String,
}

#[allow(unused)]
#[derive(Deserialize)]
struct LoginQRGenResp {
    code: i32,
    message: String,
    ttl: i32,
    data: LoginQRGenRespData,
}

#[derive(PartialEq, Debug)]
pub enum LoginQRStatue {
    NoScan,
    TimeoutOrFailure,
    Scanned,
    Success,
}

impl LoginQRStatue {
    pub fn from_code(code: i32) -> Option<Self> {
        match code {
            0 => Some(Self::Success),
            86038 => Some(Self::TimeoutOrFailure),
            86090 => Some(Self::Scanned),
            86101 => Some(Self::NoScan),
            _ => None,
        }
    }

    pub fn is_waiting(&self) -> bool {
        match self {
            Self::Success => false,
            Self::TimeoutOrFailure => false,
            Self::Scanned => true,
            Self::NoScan => true,
        }
    }

    pub fn is_success(&self) -> bool {
        match self {
            Self::Success => true,
            _ => false,
        }
    }
}

#[allow(unused)]
#[derive(Deserialize)]
struct LoginQRRefreshResp {
    code: i32,
    message: String,
    ttl: i32,
    data: LoginQRRefreshRespData,
}

#[allow(unused)]
#[derive(Deserialize)]
struct LoginQRRefreshRespData {
    url: String,
    refresh_token: String,
    timestamp: u64,
    code: i32,
    message: String,
}

pub struct LoginQR {
    key: String,
    pub statue: LoginQRStatue,
    pub qrcode_url: String,
    client: reqwest::Client,
}

impl LoginQR {
    pub async fn new() -> APIResult<Self> {
        let client = crate::get_client!()?;
        let response = client
            .get("https://passport.bilibili.com/x/passport-login/web/qrcode/generate")
            .send()
            .await?;
        let resp: LoginQRGenResp = serde_json::from_str(&response.text().await?)?;
        if resp.code != 0 {
            return Err(crate::error::Error::APIError {
                code: resp.code,
                message: resp.message,
            });
        }
        Ok(Self {
            key: resp.data.qrcode_key,
            statue: LoginQRStatue::NoScan,
            qrcode_url: resp.data.url,
            client,
        })
    }

    pub async fn refresh(&mut self) -> APIResult<Option<HashMap<String, String>>> {
        if self.statue == LoginQRStatue::Success {
            return Ok(None);
        }
        let response = self
            .client
            .get("https://passport.bilibili.com/x/passport-login/web/qrcode/poll")
            .query(&[("qrcode_key", &self.key)])
            .send()
            .await?;
        let cookies = response.cookies().fold(HashMap::new(), |mut acc, cookie| {
            acc.insert(cookie.name().to_string(), cookie.value().to_string());
            acc
        });
        let json = &response.text().await?;
        let resp: LoginQRRefreshResp = serde_json::from_str(json)?;
        if resp.code != 0 {
            return Err(crate::error::Error::APIError {
                code: resp.code,
                message: resp.message,
            });
        }
        let Some(statue) = LoginQRStatue::from_code(resp.data.code) else {
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

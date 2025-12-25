use std::fmt::Debug;

use serde::Deserialize;

pub mod const_value;
pub mod error;
pub mod user;
pub mod util;
pub mod video;

#[cfg(feature = "summon_qrcode")]
pub extern crate image;
pub extern crate reqwest;

#[macro_export]
macro_rules! get_client {
    () => {
        $crate::reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .user_agent("Mozilla/5.0 (X11; Linux x86_64; rv:146.0) Gecko/20100101 Firefox/146.0")
            .build()
    };
}

#[macro_export]
macro_rules! make_headers {
    () => {
        reqwest::header::HeaderMap::new()
    };
    ($($key:ident: $val:expr),+$(,)?) => {
        {
            let mut headers = reqwest::header::HeaderMap::new();
            $(
                headers.insert(reqwest::header::$key, reqwest::header::HeaderValue::from_static($val));
            )*
            headers
        }
    };
}

#[derive(Debug, Deserialize)]
pub struct APIResponse<D>
where
    D: Debug,
{
    pub code: i32,
    pub message: String,
    pub data: Option<D>,
}

impl<D> APIResponse<D>
where
    D: Debug,
{
    pub fn is_success(&self) -> bool {
        self.code == 0
    }

    pub fn into_result(self) -> error::APIResult<D> {
        if self.is_success() {
            Ok(self.data.unwrap())
        } else {
            Err(error::Error::APIError {
                code: self.code,
                message: self.message,
            })
        }
    }
}

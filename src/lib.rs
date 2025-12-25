use std::fmt::Debug;

use serde::Deserialize;

pub mod error;
pub mod info;
pub mod login;
pub mod user;

#[macro_export]
macro_rules! get_client {
    () => {
        reqwest::Client::builder()
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

    pub fn into_result(self) -> crate::error::APIResult<D> {
        if self.is_success() {
            Ok(self.data.unwrap())
        } else {
            Err(crate::error::Error::APIError {
                code: self.code,
                message: self.message,
            })
        }
    }
}

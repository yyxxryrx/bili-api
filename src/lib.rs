use std::fmt::Debug;

use serde::Deserialize;

pub mod const_value;
pub mod error;
pub mod sign_and_auth;
pub mod user;
pub mod util;
pub mod video;

#[cfg(feature = "summon_qrcode")]
pub extern crate image;
pub extern crate reqwest;
pub extern crate serde;

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

#[macro_export]
macro_rules! make_serde {
    ($vis:vis mod $name:ident($t:ident) { ($de:ident) => $b1:block $(, ($val:ident, $se:ident) => $b2:block)? $(,)?}) => {
        $vis mod $name {
                use super::$t;
                use $crate::serde::{Deserialize, Deserializer, Serializer};

                pub fn deserialize<'de, D>($de: D) -> Result<$t, D::Error>
                where
                    D: Deserializer<'de>,
                $b1

                $(
                pub fn serialize<S>(
                    $val: &$t,
                    $se: S,
                ) -> Result<S::Ok, S::Error>
                where
                    S: Serializer,
                $b2
                )?
            }
    };
    ($($($tt:tt)+);*;) => {
        $(
            $crate::make_serde!($($tt)+)
        )*
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

use std::time::SystemTime;

use error::SignParamsError;
use md5::{Digest, Md5};

pub struct WbiSign {
    pub img_key: String,
    pub sub_key: String,
    mixin_key: String,
}

impl WbiSign {
    pub fn new(img_key: &str, sub_key: &str) -> Self {
        Self {
            img_key: img_key.to_string(),
            sub_key: sub_key.to_string(),
            mixin_key: gen_mixin_key(img_key.to_string() + sub_key),
        }
    }

    /// 为参数签名
    ///
    /// 此函数不需要你设置 `wts` 和 `w_rid` 函数会帮你加上
    ///
    /// 此函数也不需要你排序，会帮你排序的
    pub fn sign(&self, params: &mut Vec<(&str, String)>) -> Result<(), SignParamsError> {
        let wts = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;
        params.push(("wts", wts.as_secs().to_string()));
        params.sort_by(|(k1, _), (k2, _)| k1.cmp(k2));
        params.push(("w_rid", self.get_sign(params)?));
        Ok(())
    }

    /// 为参数签名
    ///
    /// 此函数不需要你设置 `wts` 和 `w_rid` 函数会帮你加上
    ///
    /// 此函数也不需要你排序，会帮你排序的
    pub fn sign_option(
        &self,
        params: &mut Vec<(&str, Option<String>)>,
    ) -> Result<(), SignParamsError> {
        let wts = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;
        params.push(("wts", Some(wts.as_secs().to_string())));
        params.sort_by(|(k1, _), (k2, _)| k1.cmp(k2));
        params.push(("w_rid", Some(self.get_sign(params)?)));
        Ok(())
    }

    /// 此函数用于获取 `w_rid` 的值
    ///
    /// 此函数不帮你做任何事情，你需要自己设置 `wts` 和把参数按键名升序排序
    pub fn get_sign<T>(&self, params: &T) -> Result<String, SignParamsError>
    where
        T: serde::Serialize + Sized,
    {
        let args = serde_urlencoded::to_string(&params)?;
        let mut hasher = Md5::new();
        hasher.update(args + &self.mixin_key);
        let w_rid = hasher.finalize();
        Ok(hex::encode(w_rid))
    }
}

pub fn gen_mixin_key(raw_wbi_key: impl AsRef<[u8]>) -> String {
    let raw_wbi_key = raw_wbi_key.as_ref();
    let mut mixin_key = {
        let binding = crate::const_value::MIXIN_KEY_ENC_TAB
            .iter()
            // 此步操作即遍历 MIXIN_KEY_ENC_TAB，取出 raw_wbi_key 中对应位置的字符
            .map(|n| raw_wbi_key[*n as usize])
            // 并收集进数组内
            .collect::<Vec<u8>>();
        unsafe { String::from_utf8_unchecked(binding) }
    };
    let _ = mixin_key.split_off(32); // 截取前 32 位字符
    mixin_key
}

pub mod error {
    #[derive(Debug, thiserror::Error)]
    pub enum SignParamsError {
        #[error("TimeError: {0}")]
        TimeError(#[from] std::time::SystemTimeError),
        #[error("SerdeError: {0}")]
        SerdeError(#[from] serde_urlencoded::ser::Error),
        #[error("DecodeError: {0}")]
        DecodeError(#[from] std::string::FromUtf8Error),
    }
}

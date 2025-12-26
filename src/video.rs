pub mod info;
mod stream;

/// 视频ID
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum VideoID {
    Aid(u64),
    Bvid(String),
}

impl TryFrom<&str> for VideoID {
    type Error = error::VideoIDParseError;
    /// 尝试从字符串解析id
    fn try_from(id: &str) -> Result<Self, Self::Error> {
        let s = id.to_lowercase();
        if s.starts_with("av") {
            let avid = s.chars().skip(2).collect::<String>().parse::<u64>()?;
            return Ok(Self::Aid(avid));
        }
        // BV号只会出现字母和数字
        match (
            s.starts_with("bv"),
            s.chars().all(|c| c.is_ascii_alphanumeric()),
        ) {
            (true, true) => Ok(Self::Bvid(id.to_string())),
            (true, false) => Err(error::VideoIDParseError::FormatError(format!(
                "str must only contain letters and numbers, but got {id:?}"
            ))),
            _ => Err(error::VideoIDParseError::FormatError(format!(
                "str must starts with \"av\" or \"bv\" but got {id:?}"
            ))),
        }
    }
}

impl VideoID {
    /// 获取aid
    pub fn aid(&self) -> Option<u64> {
        match self {
            Self::Aid(id) => Some(*id),
            Self::Bvid(..) => None,
        }
    }

    /// 获取bvid
    pub fn bvid(&self) -> Option<&str> {
        match self {
            Self::Aid(..) => None,
            Self::Bvid(id) => Some(id),
        }
    }

    /// 将VideoID变为query对象
    pub fn to_query(&self) -> [(&'static str, Option<String>); 2] {
        [
            ("aid", self.aid().map(|v| v.to_string())),
            ("bvid", self.bvid().map(|v| v.to_string())),
        ]
    }
}

pub mod error {
    #[derive(Debug, thiserror::Error)]
    pub enum VideoIDParseError {
        #[error("ParseError: {0}")]
        ParseError(#[from] std::num::ParseIntError),
        #[error("FormatError: {0}")]
        FormatError(String),
    }
}

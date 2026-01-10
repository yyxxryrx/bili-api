use make_serde::{MakeSerde, SummonFrom};

pub mod info;

/// 剧集地区
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, SummonFrom, MakeSerde)]
pub enum SeriesRegion {
    /// 中国大陆
    China = 1,
    /// 日本
    Japan = 2,
    /// 美国
    Usa = 3,
    /// 英国
    Uk = 4,
    /// 加拿大
    Canada = 5,
    /// 中国香港
    HongKong = 6,
    /// 中国台湾
    Taiwan = 7,
    /// 韩国
    Korea = 8,
    /// 法国
    France = 9,
    /// 泰国
    Thailand = 10,
    /// 马来西亚
    Malaysia = 11,
    /// 新加坡
    Singapore = 12,
    /// 西班牙
    Spain = 13,
    /// 俄罗斯
    Russia = 14,
    /// 德国
    Germany = 15,
    /// 其他
    Other = 16,
    /// 丹麦
    Denmark = 17,
    /// 乌克兰
    Ukraine = 18,
    /// 以色列
    Israel = 19,
    /// 伊朗
    Iran = 20,
    /// 保加利亚
    Bulgaria = 21,
    /// 克罗地亚
    Croatia = 22,
    /// 冰岛
    Iceland = 23,
    /// 匈牙利
    Hungary = 24,
    /// 南非
    SouthAfrica = 25,
    /// 印尼
    Indonesia = 26,
    /// 印度
    India = 27,
    /// 哥伦比亚
    Colombia = 28,
    /// 土耳其
    Turkey = 30,
    /// 墨西哥
    Mexico = 31,
    /// 委内瑞拉
    Venezuela = 32,
    /// 巴西
    Brazil = 33,
    /// 希腊
    Greece = 34,
    /// 意大利
    Italy = 35,
    /// 挪威
    Norway = 36,
    /// 捷克
    Czech = 37,
    /// 摩洛哥
    Morocco = 38,
    /// 新西兰
    NewZealand = 39,
    /// 智利
    Chile = 40,
    /// 比利时
    Belgium = 41,
    /// 波兰
    Poland = 42,
    /// 澳大利亚
    Australia = 43,
    /// 爱尔兰
    Ireland = 44,
    /// 瑞典
    Sweden = 45,
    /// 瑞士
    Switzerland = 46,
    /// 芬兰
    Finland = 47,
    /// 苏联
    SovietUnion = 48,
    /// 荷兰
    Netherlands = 49,
    /// 越南
    Vietnam = 50,
    /// 阿根廷
    Argentina = 51,
    /// 马耳他
    Mauritius = 52,
    /// 古巴
    Cuba = 53,
    /// 菲律宾
    Philippines = 54,
    /// 哈萨克斯坦
    Kazakhstan = 55,
    /// 黎巴嫩
    Lebanon = 56,
    /// 塞浦路斯
    Cyprus = 57,
    /// 卡塔尔
    Qatar = 58,
    /// 阿联酋
    UnitedArabEmirates = 59,
    /// 奥地利
    Austria = 60,
    /// 西德
    WestGermany = 61,
    /// 卢森堡
    Luxembourg = 62,
    /// 罗马尼亚
    Romania = 63,
    /// 印度尼西亚
    Indonesia2 = 64,
    /// 南斯拉夫
    SovietUnion2 = 65,
    /// 蒙古
    Mongolia = 66,
    /// 葡萄牙
    Portugal = 70,
    /// 未知
    #[other]
    Unknown(u32),
}

/// 剧集ID
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EpisodeID {
    SeasonId(u64),
    EpId(u64),
}

impl TryFrom<&str> for EpisodeID {
    type Error = error::EpisodeIDParseError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let s = value.to_lowercase();
        if s.starts_with("ep") {
            let v = s.chars().skip(2).collect::<String>().parse::<u64>()?;
            Ok(Self::EpId(v))
        } else if s.starts_with("ss") {
            let v = s.chars().skip(2).collect::<String>().parse::<u64>()?;
            Ok(Self::SeasonId(v))
        } else {
            Err(error::EpisodeIDParseError::FormatError(value.to_string()))
        }
    }
}

impl std::str::FromStr for EpisodeID {
    type Err = error::EpisodeIDParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.try_into()
    }
}

impl From<&EpisodeID> for String {
    fn from(value: &EpisodeID) -> Self {
        match value {
            EpisodeID::EpId(id) => format!("ep{id}"),
            EpisodeID::SeasonId(id) => format!("ss{id}"),
        }
    }
}

impl std::fmt::Display for EpisodeID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from(self))
    }
}

impl EpisodeID {
    pub fn to_tuple(&self) -> (&'static str, String) {
        match self {
            Self::EpId(id) => ("ep_id", id.to_string()),
            Self::SeasonId(id) => ("season_id", id.to_string()),
        }
    }
}

mod error {
    use thiserror::Error;

    #[derive(Debug, Error)]
    pub enum EpisodeIDParseError {
        #[error("Parse Error: {0}")]
        ParseError(#[from] std::num::ParseIntError),
        #[error(r#"Format Error: must starts with "ss" or "ep", bug got "{0}""#)]
        FormatError(String),
    }
}

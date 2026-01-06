use make_serde::{MakeSerde, SummonFrom};

pub mod login;

/// 用户性别
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum UserSex {
    /// 男性
    Male,
    /// 女性
    Woman,
    /// 保密
    Secrecy,
    /// 其他，一般不会用到
    Other(String),
}

/// 这里接口只会返回这三个值，但是为了以防万一，还是做了Other来防止API变动
pub mod user_sex_serde {
    use super::UserSex;
    use serde::{Deserialize, Deserializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<UserSex, D::Error>
    where
        D: Deserializer<'de>,
    {
        let sex = String::deserialize(deserializer)?;
        Ok(match sex.as_ref() {
            "男" => UserSex::Male,
            "女" => UserSex::Woman,
            "保密" => UserSex::Secrecy,
            _ => UserSex::Other(sex),
        })
    }

    pub fn serialize<S>(sex: &UserSex, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let sex = match sex {
            UserSex::Male => "男",
            UserSex::Woman => "女",
            UserSex::Secrecy => "保密",
            UserSex::Other(sex) => &sex,
        };
        serializer.serialize_str(sex)
    }
}

/// 用户认证类型
#[derive(Debug, Clone, Copy, SummonFrom, MakeSerde)]
#[repr(u8)]
#[summon(type=u8)]
#[make_serde(type=u8)]
pub enum OfficialRole {
    /// 无
    None = 0,
    /// 个人认证 - 知名UP主
    WellKnownUP = 1,
    /// 个人认证 - 大V达人
    VerifiedInfluencer = 2,
    /// 机构认证 - 企业
    Company = 3,
    /// 机构认证 - 组织
    Organization = 4,
    /// 机构认证 - 媒体
    Media = 5,
    /// 机构认证 - 政府
    Government = 6,
    /// 个人认证 - 高能主播
    HighEnergyStreamer = 7,
    /// 个人认证 - 社会知名人士
    PublicFigure = 9,
    /// 未知（一般不会出现，除非B站改API了）
    #[other]
    Unknown(u8),
}

impl OfficialRole {
    /// 是否为个人认证
    pub fn is_personal(&self) -> bool {
        matches!(
            self,
            Self::WellKnownUP
                | Self::VerifiedInfluencer
                | Self::HighEnergyStreamer
                | Self::PublicFigure
        )
    }

    /// 是否为机构认证
    pub fn is_institutional(&self) -> bool {
        matches!(
            self,
            Self::Company | Self::Organization | Self::Media | Self::Government
        )
    }

    /// 是否为 `Unknown`
    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown(..))
    }

    /// 是否未认证（为 `None`）
    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }
}

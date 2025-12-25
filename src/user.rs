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

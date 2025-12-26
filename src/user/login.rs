use crate::make_serde;

pub mod operation;

#[derive(Debug, Clone, Copy)]
pub enum LoginSource {
    MainWeb,
    MainMini,
}

impl TryFrom<&str> for LoginSource {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "main_web" => Ok(Self::MainWeb),
            "main_mini" => Ok(Self::MainMini),
            _ => Err(value.to_string()),
        }
    }
}

impl From<&LoginSource> for &str {
    fn from(value: &LoginSource) -> Self {
        match value {
            LoginSource::MainWeb => "main_web",
            LoginSource::MainMini => "main_mini",
        }
    }
}

make_serde! {
    pub mod serde_login_source(LoginSource) {
        (deserializer) => {
            let source = String::deserialize(deserializer)?;
            source.as_str().try_into().map_err(|e| {
                serde::de::Error::custom(e)
            })
        },
        (source, serializer) => {
            serializer.serialize_str(source.into())
        }
    }
}

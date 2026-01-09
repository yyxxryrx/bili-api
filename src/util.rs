use crate::make_serde;

pub mod international;

make_serde! {
    pub mod serde_u8_2_bool(bool!) {
        (der) => {
            let v = u8::deserialize(der)?;
            match v {
                0 => Ok(false),
                1 => Ok(true),
                _ => Err(serde::de::Error::custom(format!("Expected 0 or 1, but got {v}")))
            }
        },
        (v, ser) => {
            match *v {
                true => ser.serialize_u8(1),
                false => ser.serialize_u8(0),
            }
        }
    }
}

make_serde! {
    pub mod serde_i8_2_bool(bool!) {
        (der) => {
            let v = i8::deserialize(der)?;
            match v {
                -1 => Ok(false),
                0 => Ok(true),
                _ => Err(serde::de::Error::custom(format!("Expected -1 or 0, but got {v}")))
            }
        },
        (v, ser) => {
            match *v {
                true => ser.serialize_i8(0),
                false => ser.serialize_i8(-1),
            }
        }
    }
}
use serde::{Deserialize, Deserializer, Serializer};

pub mod i32_as_bool {
    use super::*;

    pub fn serialize<S>(v: &bool, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(if *v { 1 } else { 0 })
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<bool, D::Error>
    where
        D: Deserializer<'de>,
    {
        match i32::deserialize(deserializer)? {
            0 => Ok(false),
            1 => Ok(true),
            v => {
                let s = format!("invalid value: {}, expected 0 or 1", v);
                Err(serde::de::Error::custom(s))
            }
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn i32_as_bool() -> serde_json::Result<()> {
        #[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
        struct O {
            #[serde(with = "crate::serde::i32_as_bool")]
            flag: bool,
        }

        assert!(serde_json::from_str::<O>(r#"{}"#).is_err());
        assert!(serde_json::from_str::<O>(r#"{"flag":-1}"#).is_err());
        assert!(serde_json::from_str::<O>(r#"{"flag":2}"#).is_err());

        let j1 = r#"{"flag":1}"#;
        let o1 = serde_json::from_str::<O>(j1)?;
        assert_eq!(o1, O { flag: true });
        assert_eq!(serde_json::to_string(&o1)?, j1);

        let j2 = r#"{"flag":0}"#;
        let o2 = serde_json::from_str::<O>(j2)?;
        assert_eq!(o2, O { flag: false });
        assert_eq!(serde_json::to_string(&o2)?, j2);

        Ok(())
    }
}

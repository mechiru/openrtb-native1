/// 4.1 Native Markup Request Object#Version
/// 5.1 Native Markup Response Object#Version
///
/// Version of the Native Markup version in use.
#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
pub enum Version {
    #[serde(rename = "1.0")]
    V1_0,
    #[serde(rename = "1.1")]
    V1_1,
    #[serde(rename = "1.2")]
    V1_2,
}

impl Default for Version {
    fn default() -> Self {
        Self::V1_2
    }
}

impl std::str::FromStr for Version {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn json() -> serde_json::Result<()> {
        assert!(serde_json::from_str::<Version>("0.9").is_err());

        let json = r#"["1.0","1.1","1.2"]"#;
        let e1: Vec<Version> = serde_json::from_str(json)?;
        assert_eq!(e1, vec![Version::V1_0, Version::V1_1, Version::V1_2]);
        assert_eq!(serde_json::to_string(&e1)?, json);

        Ok(())
    }
}

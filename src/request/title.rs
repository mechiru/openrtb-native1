/// 4.3 Title Request Object
///
/// The Title object is to be used for title element of the Native ad.
#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Clone)]
pub struct Title {
    /// required; integer; -
    /// Maximum length of the text in the title element. Recommended to be 25, 90, or 140.
    pub len: i32,

    /// optional; object; -
    /// This object is a placeholder that may contain custom JSON agreed to by the parties to
    /// support flexibility beyond the standard defined in this specification.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_json::Map<String, serde_json::Value>>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn json() -> serde_json::Result<()> {
        let json = r#"{"len":0}"#;
        let o1 = Title::default();
        assert_eq!(serde_json::to_string(&o1)?, json);
        assert_eq!(o1, serde_json::from_str::<Title>(json)?);

        Ok(())
    }
}

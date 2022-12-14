/// 5.3 Title Response Object
///
/// Corresponds to the Title Object in the request, with the value filled in.
/// If using assetsurl or dcourl response rather than embedded asset response, it is recommended
/// that three title objects be provided, the length of each of which is less than or equal to the
/// three recommended maximum title lengths (25,90,140).
#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Clone)]
pub struct Title<'a> {
    /// required; string; -
    /// The text associated with the text element.
    #[serde(borrow)]
    pub text: std::borrow::Cow<'a, str>,

    /// optional; integer; -
    /// The length of the title being provided. Required if using assetsurl/dcourl representation,
    /// optional if using embedded asset representation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub len: Option<i32>,

    /// optional; object; -
    /// This object is a placeholder that may contain custom JSON agreed to by the parties to
    /// support flexibility beyond the standard defined in this specification.
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_json::Map<String, serde_json::Value>>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn json() -> serde_json::Result<()> {
        let json = r#"{"text":""}"#;
        let o1 = Title::default();
        assert_eq!(serde_json::to_string(&o1)?, json);
        assert_eq!(o1, serde_json::from_str::<Title>(json)?);

        Ok(())
    }
}

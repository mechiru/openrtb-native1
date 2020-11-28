/// 5.6 Video Response Object
///
/// Corresponds to the Video Object in the request, yet containing a value of a conforming VAST tag
/// as a value.
#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Clone)]
pub struct Video<'a> {
    /// required; string; -
    /// vast xml.
    #[serde(borrow)]
    pub vasttag: std::borrow::Cow<'a, str>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn json() -> serde_json::Result<()> {
        let json = r#"{"vasttag":""}"#;
        let o1 = Video::default();
        assert_eq!(serde_json::to_string(&o1)?, json);
        assert_eq!(o1, serde_json::from_str::<Video>(json)?);

        Ok(())
    }
}

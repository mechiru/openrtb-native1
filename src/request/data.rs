/// 4.6 Data Request Object
///
/// The Data Object is to be used for all non-core elements of the native unit such as Brand Name,
/// Ratings, Review Count, Stars, Download count, descriptions etc. It is also generic for future
/// native elements not contemplated at the time of the writing of this document. In some cases,
/// additional recommendations are also included in the Data Asset Types table.
#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Clone)]
pub struct Data<'a> {
    /// required; integer; -
    /// Type ID of the element supported by the publisher. The publisher can display this
    /// information in an appropriate format. See Data Asset Types table for commonly used
    /// examples.
    pub r#type: crate::DataAssetType,

    /// optional; integer; -
    /// Maximum length of the text in the element’s response.
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
        let json = r#"{"type":1}"#;
        let o1 = Data {
            r#type: crate::DataAssetType::Sponsored,
            len: None,
            ext: None,
        };
        assert_eq!(serde_json::to_string(&o1)?, json);
        assert_eq!(o1, serde_json::from_str::<Data>(json)?);

        Ok(())
    }
}

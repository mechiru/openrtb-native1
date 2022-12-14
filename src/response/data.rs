/// 5.5 Data Response Object
///
/// Corresponds to the Data Object in the request, with the value filled in. The Data Object is to
/// be used for all miscellaneous elements of the native unit such as Brand Name, Ratings, Review
/// Count, Stars, Downloads, Price count etc. It is also generic for future native elements not
/// contemplated at the time of the writing of this document.
#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Clone)]
pub struct Data<'a> {
    /// optional; integer; -
    /// Required for assetsurl/dcourl responses, not required for embedded asset responses. The
    /// type of data element being submitted from the Data Asset Types table.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<crate::DataAssetType>,

    /// optional; integer; -
    /// Required for assetsurl/dcourl responses, not required for embedded asset responses. The
    /// length of the data element being submitted. Where applicable, must comply with the
    /// recommended maximum lengths in the Data Asset Types table.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub len: Option<i32>,

    /// required; string; -
    /// The formatted string of data to be displayed. Can contain a formatted value such as “5
    /// stars” or “$10” or “3.4 stars out of 5”.
    pub value: std::borrow::Cow<'a, str>,

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
        let json = r#"{"value":""}"#;
        let o1 = Data::default();
        assert_eq!(serde_json::to_string(&o1)?, json);
        assert_eq!(o1, serde_json::from_str::<Data>(json)?);

        Ok(())
    }
}

/// 5.4 Image Response Object
///
/// Corresponds to the Image Object in the request. The Image object to be used for all image
/// elements of the Native ad such as Icons, Main Image, etc.
/// It is recommended that if assetsurl/dcourl is being used rather than embedded assets, that an
/// image of each recommended aspect ratio (per the Image Types table) be provided for image type 3.
#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Clone)]
pub struct Image<'a> {
    /// optional; integer; -
    /// Required for assetsurl or dcourl responses, not required for embedded asset responses. The
    /// type of image element being submitted from the Image Asset Types table.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<crate::ImageAssetType>,

    /// required; string; -
    /// The text associated with the text element.
    #[serde(borrow)]
    pub url: std::borrow::Cow<'a, str>,

    /// recommended; integer; -
    /// Width of the image in pixels. Recommended for embedded asset responses. Required for
    /// assetsurl/dcourlresponses if multiple assets of same type submitted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub w: Option<i32>,

    /// recommended; integer; -
    /// Height of the image in pixels. Recommended for embedded asset responses. Required for
    /// assetsurl/dcourl responses if multiple assets of same type submitted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub h: Option<i32>,

    /// optional; object; -
    /// This object is a placeholder that may contain custom JSON agreed to by the parties to
    /// support flexibility beyond the standard defined in this specification.
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub ext: Option<json_ext::Object<'a>>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn json() -> serde_json::Result<()> {
        let json = r#"{"url":""}"#;
        let o1 = Image::default();
        assert_eq!(serde_json::to_string(&o1)?, json);
        assert_eq!(o1, serde_json::from_str::<Image>(json)?);

        Ok(())
    }
}

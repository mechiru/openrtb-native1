/// 4.4 Image Request Object
///
/// The Image object to be used for all image elements of the Native ad such as Icons, Main Image,
/// etc. Recommended sizes and aspect ratios are included in the Image Asset Types section.
#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Clone)]
pub struct Image<'a> {
    /// optional; integer; -
    /// Type ID of the image element supported by the publisher. The publisher can display this
    /// information in an appropriate format. See Table Image Asset Types.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<crate::ImageAssetType>,

    /// optional; integer; -
    /// Width of the image in pixels.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub w: Option<i32>,

    /// recommended; integer; -
    /// The minimum requested width of the image in pixels. This option should be used for any
    /// rescaling of images by the client. Either w or wmin should be transmitted. If only w is
    /// included, it should be considered an exact requirement.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wmin: Option<i32>,

    /// optional; integer; -
    /// Width of the image in pixels.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub h: Option<i32>,

    /// recommended; integer; -
    /// The minimum requested height of the image in pixels. This option should be used for any
    /// rescaling of images by the client. Either h or hmin should be transmitted. If only h is
    /// included, it should be considered an exact requirement.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hmin: Option<i32>,

    /// optional; array of string; All types allowed
    /// Whitelist of content MIME types supported. Popular MIME types include, but are not limited
    /// to “image/jpg” “image/gif”. Each implementing Exchange should have their own list of
    /// supported types in the integration docs. See Wikipedia's MIME page for more information and
    /// links to all IETF RFCs. If blank, assume all types are allowed.
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub mimes: Option<Vec<std::borrow::Cow<'a, str>>>,

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
        let json = r#"{}"#;
        let o1 = Image::default();
        assert_eq!(serde_json::to_string(&o1)?, json);
        assert_eq!(o1, serde_json::from_str::<Image>(json)?);

        Ok(())
    }
}

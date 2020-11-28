/// 5.2 Asset Response Object
///
/// Corresponds to the Asset Object in the request. The main container object for each asset
/// requested or supported by Exchange on behalf of the rendering client. Any object that is
/// required is to be flagged as such. Only one of the {title,img,video,data} objects should be
/// present in each object. All others should be null/absent. The id is to be unique within the
/// AssetObject array so that the response can be aligned.
#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Clone)]
pub struct Asset<'a> {
    /// optional; int; -
    /// Optional if assetsurl/dcourl is being used; required if embedded asset is being used.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,

    /// optional; int; 0
    /// Set to 1 if asset is required. (bidder requires it to be displayed).
    #[serde(
        default,
        skip_serializing_if = "default_ext::DefaultExt::is_default",
        with = "crate::serde::i32_as_bool"
    )]
    pub required: bool,

    /// recommended; array of object; -
    #[serde(borrow, flatten, default, skip_serializing_if = "Option::is_none")]
    pub value: Option<crate::response::AssetValue<'a>>,

    /// optional; object; -
    /// Link object for call to actions. The link object applies if the asset item is activated
    /// (clicked). If there is no link object on the asset, the parent link object on the bid
    /// response applies.
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub link: Option<crate::response::Link<'a>>,

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
        let json = r#"{}"#;
        let o1 = Asset::default();
        assert_eq!(serde_json::to_string(&o1)?, json);
        assert_eq!(o1, serde_json::from_str::<Asset>(json)?);

        Ok(())
    }
}

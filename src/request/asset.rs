/// 4.2 Asset Request Object
///
/// The main container object for each asset requested or supported by Exchange on behalf of the
/// rendering client. Any object that is required is to be flagged as such. Only one of the
/// {title,img,video,data} objects should be present in each object. All others should be
/// null/absent. The id is to be unique within the AssetObject array so that the response can be
/// aligned.
///
/// To be more explicit, it is the ID of each asset object that maps the response to the request. So
/// if a request for a title object is sent with id 1, then the response containing the title should
/// have an id of 1.
///
/// Since version 1.1 of the spec, there are recommended sizes/lengths/etc with some of the asset
/// types. The goal for asset requirements standardization is to facilitate adoption of native by
/// DSPs by limiting the diverse types/sizes/requirements of assets they must have available to
/// purchase a native ad impression. While great diversity may exist in publishers, advertisers/DSPs
/// can not be expected to provide infinite headline lengths, thumbnail aspect ratios, etc. While we
/// have not gone as far as creating a single standard, we've honed in on a few options that cover
/// the most common cases. SSPs can deviate from these standards, but should understand they may
/// limit applicable DSP demand by doing so. DSPs should feel confident that if they support these
/// standards they'll be able to access most native inventory.
#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Clone)]
pub struct Asset<'a> {
    /// required; integer; -
    /// Unique asset ID, assigned by exchange. Typically a counter for the array.
    pub id: i32,

    /// optional; integer; 0
    /// Set to 1 if asset is required. (exchange will not accept a bid without it)
    #[serde(
        default,
        skip_serializing_if = "default_ext::DefaultExt::is_default",
        with = "crate::serde::i32_as_bool"
    )]
    pub required: bool,

    // recommended; object; -
    #[serde(borrow, flatten, default, skip_serializing_if = "Option::is_none")]
    pub value: Option<crate::request::AssetValue<'a>>,

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
        let json = r#"{"id":0}"#;
        let o1 = Asset::default();
        assert_eq!(serde_json::to_string(&o1)?, json);
        assert_eq!(o1, serde_json::from_str::<Asset>(json)?);

        Ok(())
    }
}

/// 4.1 Native Markup Request Object
///
/// The Native Object defines the native advertising opportunity available for bid via this bid
/// request. It will be included as a JSON-encoded string in the bid request’s imp.native field or
/// as a direct JSON object, depending on the choice of the exchange. While OpenRTB 2.x officially
/// supports only JSON-encoded strings, many exchanges have implemented a formal object. Check with
/// your integration docs.
#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Clone)]
pub struct Request<'a> {
    /// optional; string; 1.2
    /// Version of the Native Markup version in use.
    #[serde(default)]
    pub ver: crate::Version,

    /// recommended; integer; -
    /// The context in which the ad appears. See Table of Context IDs below for a list of supported
    /// context types.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context: Option<crate::ContextType>,

    /// optional; integer; -
    /// A more detailed context in which the ad appears. See Table of Context SubType IDs below for
    /// a list of supported context subtypes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contextsubtype: Option<crate::ContextSubType>,

    /// recommended; integer; -
    /// The design/format/layout of the ad unit being offered. See Table of Placement Type IDs
    /// below for a list of supported placement types.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plcmttype: Option<crate::PlacementType>,

    /// optional; integer; 1
    /// The number of identical placements in this Layout. Refer Section 8.1 Multiplacement Bid
    /// Requests for further detail.
    #[serde(
        default = "default_plcmtcnt",
        skip_serializing_if = "is_default_plcmtcnt"
    )]
    pub plcmtcnt: i32,

    /// 0 for the first ad, 1 for the second ad, and so on. Note this would generally NOT be used
    /// in combination with plcmtcnt - either you are auctioning multiple identical placements (in
    /// which case plcmtcnt>1, seq=0) or you are holding separate auctions for distinct items in
    /// the feed (in which case plcmtcnt=1, seq=>=1)
    #[serde(default, skip_serializing_if = "default_ext::DefaultExt::is_default")]
    pub seq: i32,

    /// required; array of objects; -
    /// An array of Asset Objects. Any bid response must comply with the array of elements
    /// expressed in the bid request.
    pub assets: Vec<crate::request::Asset<'a>>,

    /// optional; integer; 0
    /// Whether the supply source / impression supports returning an assetsurl instead of an asset
    /// object. 0 or the absence of the field indicates no such support.
    #[serde(
        default,
        skip_serializing_if = "default_ext::DefaultExt::is_default",
        with = "crate::serde::i32_as_bool"
    )]
    pub aurlsupport: bool,

    /// optional; integer; 0
    /// Whether the supply source / impression supports returning a dco url instead of an asset
    /// object. 0 or the absence of the field indicates no such support. Beta feature.
    #[serde(
        default,
        skip_serializing_if = "default_ext::DefaultExt::is_default",
        with = "crate::serde::i32_as_bool"
    )]
    pub durlsupport: bool,

    /// optional; array of objects; -
    /// Specifies what type of event tracking is supported - see Event Trackers Request Object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub eventtrackers: Option<Vec<crate::request::EventTracker<'a>>>,

    /// recommended; integer; 0
    /// Set to 1 when the native ad supports buyer-specific privacy notice. Set to 0 (or field
    /// absent) when the native ad doesn’t support custom privacy links or if support is unknown.
    #[serde(
        default,
        skip_serializing_if = "default_ext::DefaultExt::is_default",
        with = "crate::serde::i32_as_bool"
    )]
    pub privacy: bool,

    /// optional; object; -
    /// This object is a placeholder that may contain custom JSON agreed to by the parties to
    /// support flexibility beyond the standard defined in this specification.
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub ext: Option<serde_json::Map<String, serde_json::Value>>,
}

impl<'a> Default for Request<'a> {
    fn default() -> Self {
        Self {
            ver: Default::default(),
            context: Default::default(),
            contextsubtype: Default::default(),
            plcmttype: Default::default(),
            plcmtcnt: default_plcmtcnt(),
            seq: Default::default(),
            assets: Default::default(),
            aurlsupport: Default::default(),
            durlsupport: Default::default(),
            eventtrackers: Default::default(),
            privacy: Default::default(),
            ext: Default::default(),
        }
    }
}

fn default_plcmtcnt() -> i32 {
    1
}

fn is_default_plcmtcnt(v: &i32) -> bool {
    *v == default_plcmtcnt()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn json() -> serde_json::Result<()> {
        let json = r#"{"ver":"1.2","assets":[]}"#;
        let o1 = Request::default();
        assert_eq!(serde_json::to_string(&o1)?, json);
        assert_eq!(o1, serde_json::from_str::<Request>(json)?);

        Ok(())
    }
}

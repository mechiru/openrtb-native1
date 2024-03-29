/// 5.1 Native Markup Response Object
///
/// The native object is the top level JSON object which identifies a native response. The native
/// object has following attributes:
#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Clone)]
pub struct Response {
    /// recommended; string; 1.2
    /// Version of the Native Markup version in use.
    #[serde(default)]
    pub ver: crate::Version,

    /// recommended; array of object; -
    /// List of native ad’s assets. Required if no assetsurl. Recommended as fallback even if
    /// assetsurl is provided.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assets: Option<Vec<crate::response::Asset>>,

    /// optional; string; -
    /// URL of an alternate source for the assets object. The expected response is a JSON object
    /// mirroring the assets object in the bid response, subject to certain requirements as
    /// specified in the individual objects. Where present, overrides the asset object in the
    /// response.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assetsurl: Option<String>,

    /// optional; string; -
    /// URL where a dynamic creative specification may be found for populating this ad, per the
    /// Dynamic Content Ads Specification. Note this is a beta option as the interpretation of the
    /// Dynamic Content Ads Specification and how to assign those elements into a native ad is
    /// outside the scope of this spec and must be agreed offline between the parties or as may be
    /// specified in a future revision of the Dynamic Content Ads spec. Where present, overrides
    /// the asset object in the response.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dcourl: Option<String>,

    /// required; object; -
    /// Destination Link. This is default link object for the ad. Individual assets can also have a
    /// link object which applies if the asset is activated(clicked). If the asset doesn’t have a
    /// link object, the parent link object applies. See LinkObject Definition.
    pub link: crate::response::Link,

    /// optional; array of string; -
    /// Array of impression tracking URLs, expected to return a 1x1 image or 204 response -
    /// typically only passed when using 3rd party trackers. To be deprecated - replaced with
    /// eventtrackers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[deprecated = "Please use eventtrackers field instead."]
    pub imptrackers: Option<Vec<String>>,

    /// optional; string; -
    /// Optional JavaScript impression tracker. This is a valid HTML, Javascript is already wrapped
    /// in <script> tags. It should be executed at impression time where it can be supported. To be
    /// deprecated - replaced with eventtrackers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[deprecated = "Please use eventtrackers field instead."]
    pub jstracker: Option<String>,

    /// optional; Array of object; -
    /// Array of tracking objects to run with the ad, in response to the declared supported methods
    /// in the request. Replaces imptrackers and jstracker, to be deprecated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub eventtrackers: Option<Vec<crate::response::EventTracker>>,

    /// optional; string; -
    /// If support was indicated in the request, URL of a page informing the user about the buyer’s
    /// targeting activity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub privacy: Option<String>,

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
        let json = r#"{"ver":"1.2","link":{"url":""}}"#;
        let o1 = Response::default();
        assert_eq!(serde_json::to_string(&o1)?, json);
        assert_eq!(o1, serde_json::from_str::<Response>(json)?);

        Ok(())
    }
}

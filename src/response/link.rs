/// 5.7 Link Response Object
///
/// Used for ‘call to action’ assets, or other links from the Native ad. This Object should be
/// associated to its peer object in the parent Asset Object or as the master link in the top level
/// Native Ad response object. When that peer object is activated (clicked) the action should take
/// the user to the location of the link.
#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Clone)]
pub struct Link<'a> {
    /// required; string; -
    /// Landing URL of the clickable link.
    #[serde(borrow)]
    pub url: std::borrow::Cow<'a, str>,

    /// optional; array of string; -
    /// List of third-party tracker URLs to be fired on click of the URL.
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub clicktrackers: Option<Vec<std::borrow::Cow<'a, str>>>,

    /// optional; string(URL); -
    /// Fallback URL for deeplink. To be used if the URL given in url is not supported by the
    /// device.
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub fallback: Option<std::borrow::Cow<'a, str>>,

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
        let json = r#"{"url":""}"#;
        let o1 = Link::default();
        assert_eq!(serde_json::to_string(&o1)?, json);
        assert_eq!(o1, serde_json::from_str::<Link>(json)?);

        Ok(())
    }
}

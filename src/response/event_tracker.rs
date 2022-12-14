/// 5.8 Event Tracker Response Object
///
/// The event trackers response is an array of objects and specifies the types of events the bidder
/// wishes to track and the URLs/information to track them. Bidder must only respond with methods
/// indicated as available in the request. Note that most javascript trackers expect to be loaded at
/// impression time, so it’s not generally recommended for the buyer to respond with javascript
/// trackers on other events, but the appropriateness of this is up to each buyer.
#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Clone)]
pub struct EventTracker {
    /// required; integer; -
    /// Type of event to track. See Event Types table.
    pub event: crate::EventType,

    /// required; integer; -
    /// Type of tracking requested. See Event Tracking Methods table.
    pub method: crate::EventTrackingMethod,

    /// optional; text; -
    /// The URL of the image or js. Required for image or js, optional for custom.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    /// optional; object containing key:value pairs; -
    /// To be agreed individually with the exchange, an array of key:value objects for custom
    /// tracking, for example the account number of the DSP with a tracking company. IE
    /// `{“accountnumber”:”123”}`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub customdata: Option<serde_json::Map<String, serde_json::Value>>,

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
        let json = r#"{"event":1,"method":1}"#;
        let o1 = EventTracker {
            event: crate::EventType::Impression,
            method: crate::EventTrackingMethod::Img,
            url: None,
            customdata: None,
            ext: None,
        };
        assert_eq!(serde_json::to_string(&o1)?, json);
        assert_eq!(o1, serde_json::from_str::<EventTracker>(json)?);

        Ok(())
    }
}

/// 4.7 Event Trackers Request Object
///
/// The event trackers object specifies the types of events the bidder can request to be tracked in
/// the bid response, and which types of tracking are available for each event type, and is included
/// as an array in the request.
#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Clone)]
pub struct EventTracker {
    /// required; integer; -
    /// Type of event available for tracking. See Event Types table.
    pub event: crate::EventType,

    /// required; array of integer; -
    /// Array of the types of tracking available for the given event. See Event Tracking Methods
    /// table.
    pub methods: Vec<crate::EventTrackingMethod>,

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
        let json = r#"{"event":1,"methods":[]}"#;
        let o1 = EventTracker {
            event: crate::EventType::Impression,
            methods: Default::default(),
            ext: Default::default(),
        };
        assert_eq!(serde_json::to_string(&o1)?, json);
        assert_eq!(o1, serde_json::from_str::<EventTracker>(json)?);

        Ok(())
    }
}

/// 7.6 Event Types Table
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum EventType {
    /// impression.
    Impression,
    /// Visible impression using MRC definition at 50% in view for 1 second.
    ViewableMrc50,
    /// 100% in view for 1 second (ie GroupM standard).
    ViewableMrc100,
    /// Visible impression for video using MRC definition at 50% in view for 2 seconds.
    ViewableVideo50,
    /// 500+
    ExchangeSpecific(i32),
}

crate::impl_enum_serde!(
    #[exchange(ident = ExchangeSpecific, greater = 500)]
    EventType {
        Impression = 1,
        ViewableMrc50 = 2,
        ViewableMrc100 = 3,
        ViewableVideo50 = 4,
    }
);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn json() -> serde_json::Result<()> {
        assert!(serde_json::from_str::<EventType>("0").is_err());
        assert!(serde_json::from_str::<EventType>("500").is_err());

        let json = "[1,2,501]";
        let e1: Vec<EventType> = serde_json::from_str(json)?;
        assert_eq!(serde_json::to_string(&e1)?, json);
        assert_eq!(
            e1,
            vec![
                EventType::Impression,
                EventType::ViewableMrc50,
                EventType::ExchangeSpecific(501),
            ]
        );

        Ok(())
    }
}

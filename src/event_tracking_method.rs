/// 7.7 Event Tracking Methods Table
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum EventTrackingMethod {
    /// Image-pixel tracking - URL provided will be inserted as a 1x1 pixel at the time of the
    /// event.
    Img,
    /// Javascript-based tracking - URL provided will be inserted as a js tag at the time of the
    /// event.
    Js,
    /// Could include custom measurement companies such as moat, doubleverify, IAS, etc - in this
    /// case additional elements will often be passed
    ExchangeSpecific(i32),
}

crate::impl_enum_serde!(
    #[exchange(ident = ExchangeSpecific, greater = 500)]
    EventTrackingMethod {
        Img = 1,
        Js = 2,
    }
);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn json() -> serde_json::Result<()> {
        assert!(serde_json::from_str::<EventTrackingMethod>("0").is_err());
        assert!(serde_json::from_str::<EventTrackingMethod>("500").is_err());

        let json = "[1,2,501]";
        let e1: Vec<EventTrackingMethod> = serde_json::from_str(json)?;
        assert_eq!(serde_json::to_string(&e1)?, json);
        assert_eq!(
            e1,
            vec![
                EventTrackingMethod::Img,
                EventTrackingMethod::Js,
                EventTrackingMethod::ExchangeSpecific(501),
            ]
        );

        Ok(())
    }
}

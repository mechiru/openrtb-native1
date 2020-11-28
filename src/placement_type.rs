/// 7.3 Placement Type IDs
///
/// The FORMAT of the ad you are purchasing, separate from the surrounding context
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PlacementType {
    /// In the feed of content - for example as an item inside the organic
    /// feed/grid/listing/carousel.
    InFeed,
    /// In the atomic unit of the content - IE in the article page or single image page.
    AtomicUnit,
    /// Outside the core content - for example in the ads section on the right rail, as a
    /// banner-style placement near the content, etc.
    Outside,
    /// Recommendation widget, most commonly presented below the article content.
    Recommendation,
    /// To be defined by the exchange
    ExchangeSpecific(i32),
}

crate::impl_enum_serde!(
    #[exchange(ident = ExchangeSpecific, greater = 500)]
    PlacementType {
        InFeed = 1,
        AtomicUnit = 2,
        Outside = 3,
        Recommendation = 4,
    }
);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn json() -> serde_json::Result<()> {
        assert!(serde_json::from_str::<PlacementType>("0").is_err());
        assert!(serde_json::from_str::<PlacementType>("500").is_err());

        let json = "[1,2,501]";
        let e1: Vec<PlacementType> = serde_json::from_str(json)?;
        assert_eq!(serde_json::to_string(&e1)?, json);
        assert_eq!(
            e1,
            vec![
                PlacementType::InFeed,
                PlacementType::AtomicUnit,
                PlacementType::ExchangeSpecific(501),
            ]
        );

        Ok(())
    }
}

/// 7.2 Context Sub Type IDs
///
/// Next-level context in which the ad appears. Again this reflects the primary context, and does
/// not imply no presence of other elements. For example, an article is likely to contain images but
/// is still first and foremost an article. SubType should only be combined with the primary context
/// type as indicated (ie for a context type of 1, only context subtypes that start with 1 are
/// valid).
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ContextSubType {
    /// General or mixed content
    GeneralOrMixed,
    /// Primarily article content (which of course could include images, etc 11 as part of the
    /// article)
    Article,
    /// Primarily video content
    Video,
    /// Primarily audio content
    Audio,
    /// Primarily image content
    Image,
    /// User-generated content - forums, comments, etc
    UserGenerated,
    /// General social content such as a general social network
    Social,
    /// Primarily email content
    Email,
    /// Primarily chat/IM content
    Chat,
    /// Content focused on selling products, whether digital or physical
    Selling,
    /// Application store/marketplace
    Marketplace,
    /// Product reviews site primarily (which may sell product secondarily)
    Review,
    /// To be defined by the exchange
    ExchangeSpecific(i32),
}

crate::impl_enum_serde!(
    #[exchange(ident = ExchangeSpecific, greater = 500)]
    ContextSubType {
        GeneralOrMixed = 10,
        Article = 11,
        Video = 12,
        Audio = 13,
        Image = 14,
        UserGenerated = 15,
        Social = 20,
        Email = 21,
        Chat = 22,
        Selling = 30,
        Marketplace = 31,
        Review = 32,
    }
);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn json() -> serde_json::Result<()> {
        assert!(serde_json::from_str::<ContextSubType>("0").is_err());
        assert!(serde_json::from_str::<ContextSubType>("500").is_err());

        let json = "[10,20,30,501]";
        let e1: Vec<ContextSubType> = serde_json::from_str(json)?;
        assert_eq!(serde_json::to_string(&e1)?, json);
        assert_eq!(
            e1,
            vec![
                ContextSubType::GeneralOrMixed,
                ContextSubType::Social,
                ContextSubType::Selling,
                ContextSubType::ExchangeSpecific(501),
            ]
        );

        Ok(())
    }
}

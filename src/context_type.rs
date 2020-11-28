/// 7.1 Context Type IDs
///
/// The context in which the ad appears - what type of content is surrounding the ad on the page at
/// a high level. This maps directly to the new Deep Dive on In-Feed Ad Units. This denotes the
/// primary context, but does not imply other content may not exist on the page - for example it's
/// expected that most content platforms have some social components, etc.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ContextType {
    /// Content-centric context such as newsfeed, article, image gallery, video gallery, or
    /// similar.
    Content,
    /// Social-centric context such as social network feed, email, chat, or similar.
    Social,
    /// Product context such as product listings, details, recommendations, reviews, or similar.
    Product,
    /// To be defined by the exchange.
    ExchangeSpecific(i32),
}

crate::impl_enum_serde!(
    #[exchange(ident = ExchangeSpecific, greater = 500)]
    ContextType {
        Content = 1,
        Social = 2,
        Product = 3,
    }
);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn json() -> serde_json::Result<()> {
        assert!(serde_json::from_str::<ContextType>("0").is_err());
        assert!(serde_json::from_str::<ContextType>("500").is_err());

        let json = "[1,2,3,501]";
        let e1: Vec<ContextType> = serde_json::from_str(json)?;
        assert_eq!(serde_json::to_string(&e1)?, json);
        assert_eq!(
            e1,
            vec![
                ContextType::Content,
                ContextType::Social,
                ContextType::Product,
                ContextType::ExchangeSpecific(501),
            ]
        );

        Ok(())
    }
}

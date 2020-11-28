/// 7.4 Data Asset Types
///
/// Below is a list of common asset element types of native advertising at the time of writing this
/// spec. This list is non-exhaustive and intended to be extended by the buyers and sellers as the
/// format evolves.
///
/// An implementing exchange may not support all asset variants or introduce new ones unique to that
/// system.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DataAssetType {
    /// Sponsored By message where response should contain the brand name of the sponsor.
    /// required; text; -; Max 25 or longer
    Sponsored,
    /// Descriptive text associated with the product or service being advertised. Longer length of
    /// text in response may be truncated or ellipsed by the exchange.
    /// recommended; text; -; Max 25 or longer
    Desc,
    /// Rating of the product being offered to the user. For example an app’s rating in an app
    /// store from 0-5.
    /// optional; number formatted as string; -; 0-5 integer formatted as string
    Rating,
    /// Number of social ratings or “likes” of the product being offered to the user.
    /// -; number formatted as string;
    Likes,
    /// Number downloads/installs of this product.
    /// -; number formatted as string;
    Downloads,
    /// Price for product / app / in-app purchase. Value should include currency symbol in
    /// localised format.
    /// -; number formatted as string;
    Price,
    /// Sale price that can be used together with price to indicate a discounted price compared to
    /// a regular price. Value should include currency symbol in localised format.
    /// -; number formatted as string;
    SalePrice,
    /// Phone number.
    /// -; formatted string
    Phone,
    /// Address.
    /// -; text
    Address,
    /// Additional descriptive text associated text with the product or service being advertised.
    /// -; text
    Desc2,
    /// Display URL for the text ad. To be used when sponsoring entity doesn’t own the content. IE
    /// sponsored by BRAND on SITE (where SITE is transmitted in this field).
    /// -; text
    DisplayUrl,
    /// CTA description - descriptive text describing a ‘call to action’ button for the destination
    /// URL.
    /// optional; text; -; Max 15 or longer
    CtaText,
    /// Reserved for Exchange specific usage numbered above 500
    /// -; unknown
    ExchangeSpecific(i32),
}

crate::impl_enum_serde!(
    #[exchange(ident = ExchangeSpecific, greater = 500)]
    DataAssetType {
        Sponsored = 1,
        Desc = 2,
        Rating = 3,
        Likes = 4,
        Downloads = 5,
        Price = 6,
        SalePrice = 7,
        Phone = 8,
        Address = 9,
        Desc2 = 10,
        DisplayUrl = 11,
        CtaText = 12,
    }
);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn json() -> serde_json::Result<()> {
        assert!(serde_json::from_str::<DataAssetType>("0").is_err());
        assert!(serde_json::from_str::<DataAssetType>("500").is_err());

        let json = "[1,2,501]";
        let e1: Vec<DataAssetType> = serde_json::from_str(json)?;
        assert_eq!(serde_json::to_string(&e1)?, json);
        assert_eq!(
            e1,
            vec![
                DataAssetType::Sponsored,
                DataAssetType::Desc,
                DataAssetType::ExchangeSpecific(501),
            ]
        );

        Ok(())
    }
}

#![allow(deprecated)]
/// 7.5 Image Asset Types
///
/// Below is a list of common image asset element types of native advertising at the time of writing
/// this spec. This list is non-exhaustive and intended to be extended by the buyers and sellers as
/// the format evolves.
///
/// An implementing exchange may not support all asset variants or may introduce new ones unique to
/// that system.
///
/// In order to facilitate adoption, recommendations are made for both minimum sizes and aspect
/// ratios. We speak here of 'minimum maximum height' or ‘max height of at least’, which means the
/// SSP should support a max height of at least this value. They are free to support larger, but the
/// DSP knows that if they have an image of this size it will be accepted. Note that SSPs will be
/// responsible for sizing image to exact size if min-max- height framework is used; exact size may
/// not be available at bid request time. Width is calculated from the 3 supported aspect ratios.
/// Note we merged the prior overlapping type 1 and type 2 as just type 1 - to be used for app icon,
/// brand logo, or similar.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ImageAssetType {
    /// Icon image.
    /// optional; -; -; max height: at least 50, aspect ratio: 1:1
    Icon,
    /// Logo image for the brand/app.
    #[deprecated = "Please use the Icon variant instead"]
    Logo,
    /// Large image preview for the ad.
    /// At least one of 2 size variants required:
    /// Small Variant: max height: 200+, max width: 200+, 267, or 382,
    ///                aspect ratio: 1:1, 4:3, or 1.91:1.
    /// Large Variant: max height: 627+, max width: 627+, 836, or 1198,
    ///                aspect ratio: 1:1, 4:3, or 1.91:1.
    Main,
    /// Reserved for Exchange specific usage numbered above 500
    ExchangeSpecific(i32),
}

crate::impl_enum_serde!(
    #[exchange(ident = ExchangeSpecific, greater = 500)]
    ImageAssetType {
        Icon = 1,
        Logo = 2,
        Main = 3,
    }
);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn json() -> serde_json::Result<()> {
        assert!(serde_json::from_str::<ImageAssetType>("0").is_err());
        assert!(serde_json::from_str::<ImageAssetType>("500").is_err());

        let json = "[1,2,501]";
        let e1: Vec<ImageAssetType> = serde_json::from_str(json)?;
        assert_eq!(serde_json::to_string(&e1)?, json);
        assert_eq!(
            e1,
            vec![
                ImageAssetType::Icon,
                ImageAssetType::Logo,
                ImageAssetType::ExchangeSpecific(501),
            ]
        );

        Ok(())
    }
}

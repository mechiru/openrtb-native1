/// 4.3 [`Title`], 4.4 [`Image`], 4.5 [`Video`], 4.6 [`Data`]
///
/// [`Title`]: ./struct.Title.html
/// [`Image`]: ./struct.Image.html
/// [`Video`]: ./struct.Video.html
/// [`Data`]: ./struct.Data.html
#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum AssetValue {
    /// recommended; object; -
    /// Title object for title assets. See TitleObject definition.
    Title(crate::request::Title),
    /// recommended; object; -
    /// Image object for image assets. See ImageObject definition.
    Img(crate::request::Image),
    /// optional1; object; -
    /// Video object for video assets. See the Video request object definition. Note that in-stream
    /// (ie preroll, etc) video ads are not part of Native. Native ads may contain a video as the
    /// ad creative itself.
    Video(crate::request::Video),
    /// recommended; object; -
    /// Data object for brand name, description, ratings, prices etc. See DataObject definition.
    Data(crate::request::Data),
}

impl AssetValue {
    pub fn is_title(&self) -> bool {
        self.as_title().is_some()
    }

    pub fn as_title(&self) -> Option<&crate::request::Title> {
        match self {
            Self::Title(title) => Some(title),
            _ => None,
        }
    }

    pub fn as_title_mut(&mut self) -> Option<&mut crate::request::Title> {
        match self {
            Self::Title(ref mut title) => Some(title),
            _ => None,
        }
    }

    pub fn is_image(&self) -> bool {
        self.as_image().is_some()
    }

    pub fn as_image(&self) -> Option<&crate::request::Image> {
        match self {
            Self::Img(image) => Some(image),
            _ => None,
        }
    }

    pub fn as_image_mut(&mut self) -> Option<&mut crate::request::Image> {
        match self {
            Self::Img(ref mut image) => Some(image),
            _ => None,
        }
    }

    pub fn is_video(&self) -> bool {
        self.as_video().is_some()
    }

    pub fn as_video(&self) -> Option<&crate::request::Video> {
        match self {
            Self::Video(video) => Some(video),
            _ => None,
        }
    }

    pub fn as_video_mut(&mut self) -> Option<&mut crate::request::Video> {
        match self {
            Self::Video(ref mut video) => Some(video),
            _ => None,
        }
    }

    pub fn is_data(&self) -> bool {
        self.as_data().is_some()
    }

    pub fn as_data(&self) -> Option<&crate::request::Data> {
        match self {
            Self::Data(data) => Some(data),
            _ => None,
        }
    }

    pub fn as_data_mut(&mut self) -> Option<&mut crate::request::Data> {
        match self {
            Self::Data(ref mut data) => Some(data),
            _ => None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn json() -> serde_json::Result<()> {
        let json = r#"{"title":{"len":0}}"#;
        let o1 = AssetValue::Title(Default::default());
        assert_eq!(serde_json::to_string(&o1)?, json);
        assert_eq!(o1, serde_json::from_str::<AssetValue>(json)?);

        Ok(())
    }
}

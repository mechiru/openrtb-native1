/// 5.3 [`Title`], 5.4 [`Image`], 5.5 [`Data`], 5.6 [`Video`]
///
/// [`Title`]: ./struct.Title.html
/// [`Image`]: ./struct.Image.html
/// [`Data`]: ./struct.Data.html
/// [`Video`]: ./struct.Video.html
#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum AssetValue<'a> {
    /// optional; object; -
    /// Title object for title assets. See TitleObject definition.
    #[serde(borrow)]
    Title(crate::response::Title<'a>),
    /// optional; object; -
    /// Image object for image assets. See ImageObject definition.
    #[serde(borrow)]
    Img(crate::response::Image<'a>),
    /// optional; object; -
    /// Video object for video assets. See Video response object definition. Note that in-stream
    /// video ads are not part of Native. Native ads may contain a video as the ad creative itself.
    #[serde(borrow)]
    Video(crate::response::Video<'a>),
    /// optional; object; -
    /// Data object for ratings, prices etc.
    #[serde(borrow)]
    Data(crate::response::Data<'a>),
}

impl<'a> AssetValue<'a> {
    pub fn is_title(&self) -> bool {
        self.as_title().is_some()
    }

    pub fn as_title(&self) -> Option<&crate::response::Title> {
        match self {
            Self::Title(title) => Some(title),
            _ => None,
        }
    }

    pub fn as_title_mut(&'a mut self) -> Option<&'a mut crate::response::Title> {
        match self {
            Self::Title(ref mut title) => Some(title),
            _ => None,
        }
    }

    pub fn is_image(&self) -> bool {
        self.as_image().is_some()
    }

    pub fn as_image(&self) -> Option<&crate::response::Image> {
        match self {
            Self::Img(image) => Some(image),
            _ => None,
        }
    }

    pub fn as_image_mut(&'a mut self) -> Option<&'a mut crate::response::Image> {
        match self {
            Self::Img(ref mut image) => Some(image),
            _ => None,
        }
    }

    pub fn is_video(&self) -> bool {
        self.as_video().is_some()
    }

    pub fn as_video(&self) -> Option<&crate::response::Video> {
        match self {
            Self::Video(video) => Some(video),
            _ => None,
        }
    }

    pub fn as_video_mut(&'a mut self) -> Option<&'a mut crate::response::Video> {
        match self {
            Self::Video(ref mut video) => Some(video),
            _ => None,
        }
    }

    pub fn is_data(&self) -> bool {
        self.as_data().is_some()
    }

    pub fn as_data(&self) -> Option<&crate::response::Data> {
        match self {
            Self::Data(data) => Some(data),
            _ => None,
        }
    }

    pub fn as_data_mut(&'a mut self) -> Option<&'a mut crate::response::Data> {
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
        let json = r#"{"title":{"text":""}}"#;
        let o1 = AssetValue::Title(Default::default());
        assert_eq!(serde_json::to_string(&o1)?, json);
        assert_eq!(o1, serde_json::from_str::<AssetValue>(json)?);

        Ok(())
    }
}

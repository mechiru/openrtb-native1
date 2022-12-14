/// 4.5 Video Request Object
///
/// The video object to be used for all video elements supported in the Native Ad. This corresponds
/// to the Video object of OpenRTB. Exchange implementers can impose their own specific
/// restrictions. Here are the required attributes of the Video Object. For optional attributes
/// please refer to OpenRTB.
#[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Clone)]
pub struct Video {
    /// required; array of string; -
    /// Content MIME types supported. Popular MIME types include, but are not limited to
    /// “video/x-ms- wmv” for Windows Media, and “video/x-flv” for Flash Video, or “video/mp4”.
    /// Note that native frequently does not support flash.
    pub mimes: Vec<String>,

    /// required; integer; -
    /// Minimum video ad duration in seconds.
    pub minduration: i32,

    /// required; integer; -
    /// Maximum video ad duration in seconds.
    pub maxduration: i32,

    /// required; array of integer; -
    /// An array of video protocols the publisher can accept in the bid response. See OpenRTB Table
    /// ‘Video Bid Response Protocols’ for a list of possible values.
    pub protocols: Vec<openrtb2::Protocol>,

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
        let json = r#"{"mimes":[],"minduration":0,"maxduration":0,"protocols":[]}"#;
        let o1 = Video::default();
        assert_eq!(serde_json::to_string(&o1)?, json);
        assert_eq!(o1, serde_json::from_str::<Video>(json)?);

        Ok(())
    }
}

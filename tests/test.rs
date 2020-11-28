macro_rules! test_json {
    ($name:ident, $path:expr, $ty:ty) => {
        #[test]
        fn $name() -> serde_json::Result<()> {
            let json = include_str!(concat!($path, ".json"));
            let output = include_str!(concat!($path, "_output.json"));
            let left = serde_json::from_str::<$ty>(json)?;
            let right = serde_json::from_str::<$ty>(output)?;
            assert_eq!(left, right);
            assert_eq!(serde_json::to_string_pretty(&left)?, output);
            Ok(())
        }
    };
}

macro_rules! test_request {
    ($name:ident, $ver:expr, $file:expr) => {
        test_json!(
            $name,
            concat!("json/", $ver, "/request/", $file),
            openrtb_native1::request::Request
        );
    };
}

macro_rules! test_response {
    ($name:ident, $ver:expr, $file:expr) => {
        test_json!(
            $name,
            concat!("json/", $ver, "/response/", $file),
            openrtb_native1::response::Response
        );
    };
}

mod v1_1 {
    // TODO: value of plcmttype is 11
    mod request {
        test_request!(social_context, "v1.1", "6.1_social_context");
        test_request!(content_context, "v1.1", "6.2_content_context");
    }

    mod response {
        test_response!(clickout, "v1.1", "6.1_clickout");
        test_response!(video, "v1.1", "6.2_video");
    }
}

mod v1_2 {
    // TODO: value of plcmttype is 11
    mod request {
        test_request!(social_context, "v1.2", "6.1_social_context");
        // TODO: .privacy="http://www.myprivacyurl.com", .eventtrackers[].{method, url}
        test_request!(content_context, "v1.2", "6.2_content_context");
        test_request!(third_party, "v1.2", "6.3_third_party");
    }

    mod response {
        test_response!(clickout, "v1.2", "6.1_clickout");
        test_response!(video, "v1.2", "6.2_video");
        test_response!(third_party, "v1.2", "6.3_third_party");
    }
}

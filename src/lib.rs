//! An implementation of [`OpenRTB Dynamic Native Ads 1.2 Final`].
//!
//! [`OpenRTB Dynamic Native Ads 1.2 Final`]: https://www.iab.com/wp-content/uploads/2018/03/OpenRTB-Native-Ads-Specification-Final-1.2.pdf

// ===== 4 request =====

pub mod request;

// ===== 4 response =====

pub mod response;

// ===== 7 enum =====

// 7.1
mod context_type;
pub use context_type::*;

// 7.2
mod context_sub_type;
pub use context_sub_type::*;

// 7.3
mod placement_type;
pub use placement_type::*;

// 7.4
mod data_asset_type;
pub use data_asset_type::*;

// 7.5
mod image_asset_type;
pub use image_asset_type::*;

// 7.6
mod event_type;
pub use event_type::*;

// 7.7
mod event_tracking_method;
pub use event_tracking_method::*;

// ===== etc =====

mod version;
pub use version::*;

// ===== internal =====

mod macros;
mod serde;

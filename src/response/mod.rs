//! 5 Native Ad Response Markup Details

// 5.1
#[allow(clippy::module_inception)]
mod response;
pub use response::*;

// 5.2
mod asset;
pub use asset::*;

// 5.3
mod title;
pub use title::*;

// 5.4
mod image;
pub use image::*;

// 5.5
mod data;
pub use data::*;

// 5.6
mod video;
pub use video::*;

// 5.7
mod link;
pub use link::*;

// 5.8
mod event_tracker;
pub use event_tracker::*;

// ===== etc =====

mod asset_value;
pub use asset_value::*;

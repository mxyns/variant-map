mod enum_map;

pub use crate::enum_map::*;
#[cfg(feature = "derive")]
pub use enum_map_derive as derive;
pub use serde;
mod enum_map;

#[cfg(feature = "macros")]
pub mod macros;

pub use crate::enum_map::*;
#[cfg(feature = "derive")]
pub use enum_map_derive as derive;
pub use serde;

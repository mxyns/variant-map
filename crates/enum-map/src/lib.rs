pub mod common;
pub use serde;

pub mod hashmap {
    mod lib;
    pub use lib::*;
}

pub mod btreemap {
    mod lib;
    pub use lib::*;
}

#[cfg(feature = "derive")]
pub use enum_map_derive as derive;

#[cfg(feature = "macros")]
pub mod macros;

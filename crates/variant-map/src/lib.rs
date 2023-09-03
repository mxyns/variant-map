//! Enum variants stored in Maps.
//!
//! Provides different kinds of map-equivalent types to store enum variants into.
//! As those data structures are maps, they store one value of each variant.
//! All Maps are [serde::Serialize]-able and [serde::Deserialize]-able
//!
//!
//! Those maps can be generated easily using the derive macros from [variant_map_derive].
//! [variant_map_derive] provides a derive macro for a `StructMap` (a struct type with a field per enum variant).
//! [variant_map_derive] can be included using the `derive` feature on [variant_map][crate]
//!
//!
//! This crate also provide simple [macros] to lighten the syntax.
//!
//!
//! # Example
//!
//! ```
//!     use variant_map_derive::VariantStore;
//!
//!     #[derive(VariantStore)]
//!     enum MyEnum {
//!         A,
//!         B(i32),
//!     }
//!
//!     fn main() {
//!         use variant_map::{as_key, as_map};
//!         let mut map = <as_map!(MyEnum)>::default();
//!         let _: &MyEnum = map.get(&<as_key!(MyEnum)>::A).unwrap();
//!         let _: &MyEnum = map.get(&MyEnumKey::A).unwrap();
//!         map[&MyEnumKey::B] = MyEnum::B(69);
//!     }
//! ```
//!
//! For more customizability of the [Map][common::MapValue::Map] check out the [variant_map_derive] crate documentation
//!
//! For more detailed examples check out the [example project](https://github.com/mxyns/variant-map/tree/master/example) on this crates' [repo](https://github.com/mxyns/variant-map/)

/// Code in common between [hashmap] and [btreemap]
pub mod common;

/// Used by the [variant_map_derive] to provide [serde::Serialize] and [serde::Deserialize] implementations
pub use serde;

/// A [hashmap::Map] storing Enum variants based on a [std::collections::HashMap]
pub mod hashmap {
    mod lib;
    pub use lib::*;
}

/// A [btreemap::Map] storing Enum variants based on a [std::collections::BTreeMap]
pub mod btreemap {
    mod lib;
    pub use lib::*;
}

/// Derive macro which derives an enum of keys and implements [common::MapValue] on your enum
/// Available when using the *derive* or *struct-map* feature
#[cfg(feature = "derive")]
pub use variant_map_derive as derive;

/// Helper [macros] to access the associated [Map][common::MapValue::Map] and [Key][common::MapValue::Key] types to an enum implementing [MapValue][common::MapValue]
/// Available when using the *macros* feature
#[cfg(feature = "macros")]
pub mod macros;

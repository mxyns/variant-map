/// Shorthand notation for <Enum as [MapValue][crate::common::MapValue]>::[Key][crate::common::MapValue::Key]
/// Used to access the [Key][crate::common::MapValue::Key] enum of an Enum implementing [MapValue][crate::common::MapValue]
///
/// # Example
///
/// ```
///     use variant_map_derive::VariantStore;
///
///     #[derive(VariantStore)]
///     enum MyEnum {
///         A,
///         B(i32),
///     }
///
///     fn main() {
///         use variant_map::{as_key};
///         let key = <as_key!(MyEnum)>::A;
///     }
///
///
#[macro_export]
macro_rules! as_key {
    ($T:ty) => {
        <$T as MapValue>::Key
    };
    ($T:ty, $V:ident) => {
        <$T as MapValue>::Key::$V
    };
}
pub use as_key;

/// Shorthand notation for <Enum as [MapValue][crate::common::MapValue]>::[Map][crate::common::MapValue::Map]
/// Used to access the [Key][crate::common::MapValue::Key] enum of an Enum implementing [MapValue][crate::common::MapValue]
///
/// # Example
///
/// ```
///     use variant_map_derive::VariantStore;
///
///     #[derive(VariantStore)]
///     enum MyEnum {
///         A,
///         B(i32),
///     }
///
///     fn main() {
///         use variant_map::{as_map};
///         let map = <as_map!(MyEnum)>::default();
///     }
///
///
#[macro_export]
macro_rules! as_map {
    ($T:ty) => {
        <$T as MapValue>::Map
    };
}
pub use as_map;

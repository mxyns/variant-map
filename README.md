# variant-map

Enum variants stored in Maps.

Provides different kinds of map-equivalent types to store enum variants into.
As those data structures are maps, they store one value of each variant.
All Maps are *serde::Serialize*-able and *serde::Deserialize*-able

Those maps can be generated easily using the derive macros from [variant_map_derive](/crates/variant-map-derive).
[variant_map_derive](/crates/variant-map-derive) provides a derive macro for a `StructMap` (a struct type with a field per enum variant).
[variant_map_derive](/crates/variant-map-derive) can be included using the `derive` feature of [variant_map](/crates/variant-map)


This crate also provide simple **macros** to lighten the syntax with the `macros` feature.

## Main crate (GUI App)
[variant-map](/crates/variant-map)

 # Example

```rust
 use variant_map_derive::VariantStore;

 #[derive(VariantStore)]
 enum MyEnum {
     A,
     B(i32),
 }

 fn main() {
     use variant_map::{as_key, as_map};
     let mut map = <as_map!(MyEnum)>::default();
     let _: &MyEnum = map.get(&<as_key!(MyEnum)>::A).unwrap();
     let _: &MyEnum = map.get(&MyEnumKey::A).unwrap();
     map[&MyEnumKey::B] = MyEnum::B(69);
 }
 ```

For detailed examples check out the [example project](https://github.com/mxyns/variant-map/tree/master/example) on this crates' [repo](https://github.com/mxyns/variant-map/)

## Sub-crate
* [variant-map-derive](/crates/variant-map-derive): derive macro for [variant-map](/crates/variant-map) with an added [`StructMap`](/crates/variant-map-derive/src/structs.rs) store type
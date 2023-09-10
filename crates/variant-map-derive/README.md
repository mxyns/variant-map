# variant-map-derive

Enum variants stored in Maps.

Provides derive macros for `variant_map`

Includes a `StructMap` which is a struct with a field per variant of the enum

Pro: This struct has instant access to the fields (compared to the other Maps that need a lookup)

Con: Restricted API

# Example

```rust
 use variant_map_derive::VariantStore;

 #[derive(VariantStore)]
 enum MyEnum {
     A,
     B(i32),
 }
```

For more detailed examples check out the [example project](https://github.com/mxyns/variant-map/tree/master/example) on
this crates' [repo](https://github.com/mxyns/variant-map/)

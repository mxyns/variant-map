//! Example project showcasing the use of the [variant_map] and [variant_map_derive] crates.
//!
//! Each function shows a use of the library for the kind of (enum, map) pair in its name.

#[allow(unused_imports)]
use variant_map::derive as variant_map_derive;

use variant_map;
use variant_map::common::MapValue;
use variant_map::derive::{VariantStore};
use variant_map::{as_key, as_map};
use serde::{Deserialize, Serialize};

/// A classic enum with unit and tuple variants
/// [variant_map::btreemap::Map] using a [std::collections::BTreeMap] for this enum is derived and used
pub fn normal_enum() {
    #[derive(Serialize, Deserialize, VariantStore)]
    #[VariantStore(datastruct = "BTreeMap", visibility="out-of-scope", keys(name = "TestKeys", derive(::serde::Serialize, ::serde::Deserialize)))]
    enum TestEnum {
        A,
        B,
        C(i32),
        #[key_name(code = "Dimitri", serde = "dimitri")]
        #[serde(rename = "dimitri")]
        D(i32, u64, (u16, String)),
    }

    let mut map: as_map!(TestEnum) = TestEnum::make_map();
    map.insert(TestEnum::A);
    map.insert(TestEnum::B);
    map.insert(TestEnum::C(0));
    map.insert(TestEnum::D(0, 1, (2, "mdr".to_string())));
    let _k = <as_key!(TestEnum)>::A;
    let _k = as_key!(TestEnum, A);
    let _a = map.get(&<TestEnum as MapValue>::Key::A);
    let _d = map.get(&<TestEnum as MapValue>::Key::Dimitri);
    let _b = &map[<TestEnum as MapValue>::Key::B];
    let _b = &mut map[<TestEnum as MapValue>::Key::B];

    println!("{}", serde_json::to_string(&map).unwrap());
}

/// A generic enum with unit and tuple variants
/// [variant_map::hashmap::Map] using a [std::collections::HashMap] for this enum is derived and used
pub fn generic_enum() {
    trait UselessTrait {}
    trait VeryUselessTrait {}
    #[derive(Serialize, Deserialize, VariantStore)]
    #[VariantStore(datastruct = "HashMap", keys(name = "TestKeys", derive(::serde::Serialize, ::serde::Deserialize)))]
    enum GenericEnum<T: VeryUselessTrait>
    where
        T: UselessTrait,
    {
        A,
        B,
        C(i32),
        #[key_name(code = "Dimitri", serde = "dimitri")]
        #[serde(rename = "dimitri")]
        D(i32, T, (u16, String)),
    }

    #[derive(Serialize, Deserialize)]
    struct G {}

    impl UselessTrait for G {}
    impl VeryUselessTrait for G {}

    let mut map: as_map!(GenericEnum<G>) = GenericEnum::make_map();
    map.insert(GenericEnum::A);
    map.insert(GenericEnum::B);
    map.insert(GenericEnum::C(0));
    map.insert(GenericEnum::D(0, G {}, (2, "mdr".to_string())));
    let _k = <as_key!(GenericEnum<G>)>::A;
    let _k = as_key!(GenericEnum<G>, A);
    let _a = map.get(&<GenericEnum<G> as MapValue>::Key::A);
    let _d = map.get(&<GenericEnum<G> as MapValue>::Key::Dimitri);
    let _b = &map[<GenericEnum<G> as MapValue>::Key::B];
    let _b = &mut map[<GenericEnum<G> as MapValue>::Key::B];

    println!("{}", serde_json::to_string(&map).unwrap());
}

/// A classic enum with unit and tuple variants
/// Custom Struct with a field per variant for this enum is derived and used
pub fn normal_enum_struct_map() {
    #[derive(Debug, Clone, Serialize, Deserialize, VariantStore)]
    #[VariantStore(datastruct = "StructMap", keys(name = "TestKeys", derive(::serde::Serialize, ::serde::Deserialize)))]
    #[VariantStruct(derive(Clone), features(index, serialize, deserialize))]
    enum TestEnum {
        A,
        B,
        C(i32),
        #[key_name(code = "Dimitri", serde = "haha")]
        #[serde(rename = "dimitri")]
        D(i32, u64, (u16, String)),
    }

    let mut map: as_map!(TestEnum) = TestEnum::make_map();
    map.insert(TestEnum::A);
    map.insert(TestEnum::B);
    map.insert(TestEnum::C(0));
    map.insert(TestEnum::D(0, 1, (2, "mdr".to_string())));
    let _k = <as_key!(TestEnum)>::A;
    let _k = as_key!(TestEnum, A);
    let _a = map.get(&<TestEnum as MapValue>::Key::A);
    let _d = map.get(&<TestEnum as MapValue>::Key::Dimitri);
    let _b = &map[<TestEnum as MapValue>::Key::B];
    let _b = &mut map[<TestEnum as MapValue>::Key::B];

    println!("{}", serde_json::to_string(&map).unwrap());
}

/// A generic enum with unit and tuple variants
/// Custom Struct with a field per variant for this enum is derived and used
pub fn generic_enum_struct_map() {
    trait UselessTrait {}
    trait SuperUselessTrait {}
    #[derive(Debug, Serialize, Deserialize, VariantStore)]
    #[VariantStore(datastruct = "StructMap", keys(name = "TestKeys", derive(::serde::Serialize, ::serde::Deserialize)))]
    #[VariantStruct(name = "TestStructMap", features(index, serialize, deserialize))]
    enum TestEnum<T: UselessTrait> where T: SuperUselessTrait {
        A,
        B,
        C(i32),
        #[key_name(code = "Dimitri", serde = "dimitri")]
        #[serde(rename = "dimitri")]
        D(i32, T, (u16, String)),
    }
    impl<T> UselessTrait for T {}
    impl<T> SuperUselessTrait for T {}

    let mut map: as_map!(TestEnum<i64>) = TestEnum::<i64>::make_map();
    map.insert(TestEnum::A);
    map.insert(TestEnum::B);
    map.insert(TestEnum::C(0));
    map.insert(TestEnum::D(0, 1, (2, "mdr".to_string())));
    let _k = <as_key!(TestEnum<i64>)>::A;
    let _k = as_key!(TestEnum<i64>, A);
    let _a = map.get(&<TestEnum<i64> as MapValue>::Key::A);
    let _d = map.get(&<TestEnum<i64> as MapValue>::Key::Dimitri);
    let _b = &map[<TestEnum<i64> as MapValue>::Key::B];
    let _b = &mut map[<TestEnum<i64> as MapValue>::Key::B];

    println!("{}", serde_json::to_string(&map).unwrap());
}


#[doc(hidden)]
fn main() {
    normal_enum();
    generic_enum();
    normal_enum_struct_map();
    generic_enum_struct_map();
}

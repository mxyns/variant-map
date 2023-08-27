use enum_map::common::MapValue;
use enum_map::derive::EnumMap;
use enum_map::{as_key, as_map};
use serde::{Deserialize, Serialize};

fn normal_enum() {
    #[derive(Serialize, Deserialize, EnumMap)]
    #[EnumMap(name = "TestKeys", map = "BTreeMap")]
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
}

fn generic_enum() {
    trait UselessTrait {}
    trait VeryUselessTrait {}
    #[derive(Serialize, Deserialize, EnumMap)]
    #[EnumMap(name = "TestKeys", map = "BTreeMap")]
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
}

fn enum_struct_map() {

    #[derive(Debug, Serialize, Deserialize, EnumMap)]
    #[EnumMap(name = "TestKeys", map = "StructMap")]
    enum TestEnum {
        A,
        B,
        C(i32),
        #[key_name(code = "Dimitri", serde = "dimitri")]
        #[serde(rename = "dimitri")]
        D(i32, u64, (u16, String)),
    }
/*
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
    */
}

fn main() {
    normal_enum();
    generic_enum();
    enum_struct_map();
}

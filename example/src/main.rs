use enum_map::common::MapValue;
use enum_map::derive::EnumMap;
use enum_map::{as_key, as_map};
use serde::{Deserialize, Serialize};

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

fn main() {
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

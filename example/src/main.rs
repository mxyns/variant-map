use enum_map::derive::EnumMap;
use enum_map::EnumMapValue;
use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize, EnumMap, Debug)]
#[EnumMap(name = "TestKeys")]
enum TestEnum {
    A,
    B,
    C(i32),
    #[key_name(code = "Dimitri", serde = "dimitri")]
    #[serde(rename = "dimitri")]
    D(i32, u64, (u16, String)),
}

fn main() {
    let mut map = TestEnum::make_map();
    map.insert(TestEnum::A);
    map.insert(TestEnum::B);
    map.insert(TestEnum::C(0));
    map.insert(TestEnum::D(0, 1, (2, "mdr".to_string())));
    let _a = map.get(&<TestEnum as EnumMapValue>::Key::A);
    let _d = map.get(&<TestEnum as EnumMapValue>::Key::Dimitri);
    let _b = &map[<TestEnum as EnumMapValue>::Key::B];
    let _b = &mut map[<TestEnum as EnumMapValue>::Key::B];
}

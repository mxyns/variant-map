use enum_map::derive::EnumMap;
use enum_map::EnumMapValue;
use serde::Serialize;
#[derive(Serialize, EnumMap)]
#[EnumMap(name="TestKeys")]
enum TestEnum {
    A,
    B,
    C(i32),
    #[key_name(code = "Dimitri", serde = "dimitri")]
    #[serde(rename = "dimitri")]
    D(i32, u64, (u16, &'static str)),
}

fn main() {
    let mut map = TestEnum::make_map();
    map.insert(TestEnum::A);
    map.insert(TestEnum::B);
    map.insert(TestEnum::C(0));
    map.insert(TestEnum::D(0, 1, (2, "mdr")));
    let _a = map.get(&<TestEnum as EnumMapValue>::Key::A);
    let _d = map.get(&<TestEnum as EnumMapValue>::Key::Dimitri);
}

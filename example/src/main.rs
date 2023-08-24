use serde::Serialize;
use enum_map::derive::EnumMap;
use enum_map::{EnumMapValue};
#[derive(Serialize, EnumMap)]
enum TestEnum {
    A,
    B,
    C(i32),
    D(i32, u64, (u16, &'static str)),
}

fn main() {
    let mut map = TestEnum::make_map();
    map.insert(TestEnum::A);
    map.insert(TestEnum::B);
    map.insert(TestEnum::C(0));
    map.insert(TestEnum::D(0, 1, (2, "mdr")));
    let _a = map.get(&<TestEnum as EnumMapValue>::Key::A);
    let _d = map.get(&<TestEnum as EnumMapValue>::Key::D);
}

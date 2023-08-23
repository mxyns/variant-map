use enum_map_derive::EnumMap;

#[test]
fn test() {

    #[derive(EnumMap)]
    enum TestEnum {
        A,
        B,
        C(i32),
        D(i32, u64, (u16, String))
    }

    let mut map = TestEnum::make_map();
    map.insert(TestEnum::A);

    serde_json::to_string(&map);
}
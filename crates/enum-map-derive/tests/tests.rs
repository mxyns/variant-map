use enum_map_derive::EnumMap;

#[test]
fn test() {

    #[derive(EnumMap)]
    enum TestEnum {
        A,
        B,
        C(i32),
        D(i32)
    }

}
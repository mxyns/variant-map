mod maps_tests;
mod struct_tests;

#[test]
fn maps_tests() {
    maps_tests::ensure_correct_key();
    maps_tests::insert_get_map();
    maps_tests::serialize();
}

#[test]
fn struct_tests() {
    struct_tests::ensure_correct_key();
    struct_tests::insert_get_map();
    struct_tests::serialize();
}

mod maps_tests;
mod struct_tests;

#[test]
fn maps_tests() {
    maps_tests::insert_get_map();
    maps_tests::serialize();
    maps_tests::ensure_correct_key();
}

#[test]
fn struct_tests() {

}

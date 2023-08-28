pub trait MapValue: Sized {
    type Key;
    type Map;

    fn to_key(&self) -> Self::Key;

    fn make_map() -> Self::Map;
}

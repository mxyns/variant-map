#[macro_export]
macro_rules! as_key {
    ($T:ty) => {
        <$T as EnumMapValue>::Key
    };
    ($T:ty, $V:ident) => {
        <$T as EnumMapValue>::Key::$V
    };
}
pub use as_key;

#[macro_export]
macro_rules! as_map {
    ($T:ty) => {
        <$T as EnumMapValue>::Map
    };
}
pub use as_map;
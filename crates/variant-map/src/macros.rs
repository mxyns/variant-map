#[macro_export]
macro_rules! as_key {
    ($T:ty) => {
        <$T as MapValue>::Key
    };
    ($T:ty, $V:ident) => {
        <$T as MapValue>::Key::$V
    };
}
pub use as_key;

#[macro_export]
macro_rules! as_map {
    ($T:ty) => {
        <$T as MapValue>::Map
    };
}
pub use as_map;

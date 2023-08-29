/// Implement this trait on your enum to bind a [Map][MapValue::Map] and [Key][MapValue::Key] type to it
pub trait MapValue: Sized {

    /// Associated Key type (most likely an enum of unit variants)
    type Key;

    /// Map allowing 1-to-1 mapping between [Keys][MapValue::Key] and [Value][Self] (Self)
    type Map;

    /// Match each enum variant to a [Key][MapValue::Key]
    fn to_key(&self) -> Self::Key;

    /// Initialize an empty [Map][MapValue::Map]
    fn make_map() -> Self::Map;
}

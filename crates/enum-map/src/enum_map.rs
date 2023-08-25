use serde::de::{DeserializeOwned, SeqAccess, Visitor};
use serde::ser::SerializeSeq;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashMap;
use std::fmt::Formatter;
use std::hash::Hash;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut, Index, IndexMut};

pub trait EnumMapValue: Sized {
    type Key;
    type Map;

    fn to_key(&self) -> Self::Key;

    fn make_map() -> Self::Map;
}

impl<K, V> EnumMap<K, V>
where
    K: HashKey,
{
    pub fn insert(&mut self, value: V) -> Option<V>
    where
        K: PartialEq + Hash,
        V: EnumMapValue<Key = K>,
    {
        let key: K = value.to_key();
        self.inner.insert(key, value)
    }
}

pub trait HashKey: Eq + Hash {}

#[derive(Debug)]
pub struct EnumMap<Key, Value>
where
    Key: HashKey,
{
    inner: HashMap<Key, Value>,
}

impl<Key, Value> From<HashMap<Key, Value>> for EnumMap<Key, Value>
where
    Key: HashKey,
{
    fn from(value: HashMap<Key, Value>) -> Self {
        Self { inner: value }
    }
}

impl<Key, Value> EnumMap<Key, Value>
where
    Key: HashKey,
{
    pub fn new(map: HashMap<Key, Value>) -> Self {
        EnumMap { inner: map }
    }
}

impl<Key, Value> Default for EnumMap<Key, Value>
where
    Key: HashKey,
{
    fn default() -> Self {
        EnumMap {
            inner: HashMap::new(),
        }
    }
}

impl<Key, Value> Serialize for EnumMap<Key, Value>
where
    Key: HashKey,
    Value: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_seq(Some(self.len()))?;

        for v in self.deref().values() {
            map.serialize_element(v)?
        }

        map.end()
    }
}

struct EnumMapVisitor<Key, Value>
where
    Key: HashKey,
{
    marker: PhantomData<fn() -> EnumMap<Key, Value>>,
}

impl<'de, Key, Value> Visitor<'de> for EnumMapVisitor<Key, Value>
where
    Key: HashKey,
    Value: EnumMapValue<Key = Key> + DeserializeOwned,
{
    type Value = EnumMap<Key, Value>;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        write!(formatter, "EnumMapVisitor expects to receive a map of <EnumKey, Enum> with untagged Enum variants and EnumKey serializing to Enum variants' names ")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let map_size = seq.size_hint().unwrap_or(0);
        let mut map: HashMap<Key, Value> = HashMap::<Key, Value>::with_capacity(map_size);

        while let Some(value) = seq.next_element()? {
            let variant: Value = value;
            map.insert(variant.to_key(), variant);
        }

        Ok(EnumMap::from(map))
    }
}

impl<'de, Key, Value> Deserialize<'de> for EnumMap<Key, Value>
where
    Key: HashKey,
    Value: EnumMapValue<Key = Key> + DeserializeOwned,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let visitor = EnumMapVisitor::<Key, Value> {
            marker: PhantomData,
        };
        deserializer.deserialize_seq(visitor)
    }
}

impl<Key, Value> Deref for EnumMap<Key, Value>
where
    Key: HashKey,
{
    type Target = HashMap<Key, Value>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<Key, Value> DerefMut for EnumMap<Key, Value>
where
    Key: HashKey,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<Key, Value> Index<Key> for EnumMap<Key, Value>
where
    Key: HashKey,
{
    type Output = Value;

    fn index(&self, index: Key) -> &Self::Output {
        self.inner.get(&index).unwrap()
    }
}

impl<Key, Value> IndexMut<Key> for EnumMap<Key, Value>
where
    Key: HashKey,
{
    fn index_mut(&mut self, index: Key) -> &mut Self::Output {
        self.inner.get_mut(&index).unwrap()
    }
}

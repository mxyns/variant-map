use serde::de::{DeserializeOwned, SeqAccess, Visitor};
use serde::ser::SerializeSeq;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::BTreeMap;
use std::fmt::Formatter;
use std::hash::Hash;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut, Index, IndexMut};

use crate::common::MapValue;

pub trait OrdHashKey: Ord + Eq + Hash {}

#[derive(Debug)]
pub struct Map<Key, Value>
where
    Key: OrdHashKey,
{
    inner: BTreeMap<Key, Value>,
}

impl<K, V> Map<K, V>
where
    K: OrdHashKey,
{
    pub fn insert(&mut self, value: V) -> Option<V>
    where
        K: OrdHashKey,
        V: MapValue<Key = K>,
    {
        let key: K = value.to_key();
        self.inner.insert(key, value)
    }
}

impl<Key, Value> From<BTreeMap<Key, Value>> for Map<Key, Value>
where
    Key: OrdHashKey,
{
    fn from(value: BTreeMap<Key, Value>) -> Self {
        Map::new(value)
    }
}

impl<Key, Value> Map<Key, Value>
where
    Key: OrdHashKey,
{
    pub fn new(map: BTreeMap<Key, Value>) -> Self {
        Map { inner: map }
    }
}

impl<Key, Value> Default for Map<Key, Value>
where
    Key: OrdHashKey,
{
    fn default() -> Self {
        Map {
            inner: BTreeMap::new(),
        }
    }
}

impl<Key, Value> Serialize for Map<Key, Value>
where
    Key: OrdHashKey,
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
    Key: OrdHashKey,
{
    marker: PhantomData<fn() -> Map<Key, Value>>,
}

impl<'de, Key, Value> Visitor<'de> for EnumMapVisitor<Key, Value>
where
    Key: OrdHashKey,
    Value: MapValue<Key = Key> + DeserializeOwned,
{
    type Value = Map<Key, Value>;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        write!(formatter, "EnumMapVisitor expects to receive a map of <EnumKey, Enum> with untagged Enum variants and EnumKey serializing to Enum variants' names ")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut map: BTreeMap<Key, Value> = BTreeMap::<Key, Value>::new();

        while let Some(value) = seq.next_element()? {
            let variant: Value = value;
            map.insert(variant.to_key(), variant);
        }

        Ok(Map::from(map))
    }
}

impl<'de, Key, Value> Deserialize<'de> for Map<Key, Value>
where
    Key: OrdHashKey,
    Value: MapValue<Key = Key> + DeserializeOwned,
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

impl<Key, Value> Deref for Map<Key, Value>
where
    Key: OrdHashKey,
{
    type Target = BTreeMap<Key, Value>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<Key, Value> DerefMut for Map<Key, Value>
where
    Key: OrdHashKey,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<Key, Value> Index<Key> for Map<Key, Value>
where
    Key: OrdHashKey,
{
    type Output = Value;

    fn index(&self, index: Key) -> &Self::Output {
        self.inner.get(&index).unwrap()
    }
}

impl<Key, Value> IndexMut<Key> for Map<Key, Value>
where
    Key: OrdHashKey,
{
    fn index_mut(&mut self, index: Key) -> &mut Self::Output {
        self.inner.get_mut(&index).unwrap()
    }
}

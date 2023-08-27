use serde_json::{Map as SerdeMap, Value};
use user::{MyEnum, MyEnumKey};
use enum_map::common::MapValue;


#[allow(non_snake_case, dead_code)]
mod user {
    use enum_map::common::MapValue;
    use serde::de::{SeqAccess, Visitor};
    use serde::ser::SerializeSeq;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use std::fmt::{Debug, Formatter};
    use std::mem;
    use std::ops::{Index, IndexMut};

    #[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub enum MyEnum {
        A,
        B(i32),
        C,
        D(i64),
    }

    #[derive(Debug, PartialEq, Eq)]
    pub enum MyEnumKey {
        A,
        B,
        C,
        D,
    }

    #[derive(Debug, Default)]
    pub struct MyStruct {
        A: Option<MyEnum>,
        B: Option<MyEnum>,
        C: Option<MyEnum>,
        D: Option<MyEnum>,
    }

    impl Serialize for MyStruct {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let mut seq = serializer.serialize_seq(Some(4))?;

            if let Some(ref A) = self.A {
                seq.serialize_element(A)?
            }
            if let Some(ref B) = self.B {
                seq.serialize_element(B)?
            }
            if let Some(ref C) = self.C {
                seq.serialize_element(C)?
            }
            if let Some(ref D) = self.D {
                seq.serialize_element(D)?
            }

            seq.end()
        }
    }

    struct MyStructVisitor;

    impl<'de> Visitor<'de> for MyStructVisitor {
        type Value = MyStruct;

        fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
            formatter.write_str("MyStructVisitor expects a MyStruct holding MyEnum variants")
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: SeqAccess<'de>,
        {
            let mut result = MyStruct::default();

            while let Some(elem) = seq.next_element::<Option<MyEnum>>()? {
                match elem {
                    Some(MyEnum::A) => result.A = elem,
                    Some(MyEnum::B(_)) => result.B = elem,
                    Some(MyEnum::C) => result.C = elem,
                    Some(MyEnum::D(_)) => result.D = elem,
                    None => {}
                }
            }

            Ok(result)
        }
    }
    impl<'de> Deserialize<'de> for MyStruct {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_seq(MyStructVisitor {})
        }
    }

    impl MyStruct {
        pub fn remove(&mut self, key: &MyEnumKey) -> Option<MyEnum> {
            match key {
                MyEnumKey::A => mem::take(&mut self.A),
                MyEnumKey::B => mem::take(&mut self.B),
                MyEnumKey::C => mem::take(&mut self.C),
                MyEnumKey::D => mem::take(&mut self.D),
            }
        }

        pub fn insert(&mut self, value: MyEnum) -> Option<MyEnum> {
            match value {
                MyEnum::A => mem::replace(&mut self.A, Some(value)),
                MyEnum::B(_) => mem::replace(&mut self.B, Some(value)),
                MyEnum::C => mem::replace(&mut self.C, Some(value)),
                MyEnum::D(_) => mem::replace(&mut self.D, Some(value)),
            }
        }

        pub fn get(&self, key: &MyEnumKey) -> &Option<MyEnum> {
            match key {
                MyEnumKey::A => &self.A,
                MyEnumKey::B => &self.B,
                MyEnumKey::C => &self.C,
                MyEnumKey::D => &self.D,
            }
        }

        pub fn get_mut(&mut self, key: &MyEnumKey) -> &mut Option<MyEnum> {
            match key {
                MyEnumKey::A => &mut self.A,
                MyEnumKey::B => &mut self.B,
                MyEnumKey::C => &mut self.C,
                MyEnumKey::D => &mut self.D,
            }
        }
    }

    impl Index<MyEnumKey> for MyStruct {
        type Output = Option<MyEnum>;

        fn index(&self, key: MyEnumKey) -> &Self::Output {
            match key {
                MyEnumKey::A => &self.A,
                MyEnumKey::B => &self.B,
                MyEnumKey::C => &self.C,
                MyEnumKey::D => &self.D,
            }
        }
    }

    impl IndexMut<MyEnumKey> for MyStruct {
        fn index_mut(&mut self, key: MyEnumKey) -> &mut Self::Output {
            match key {
                MyEnumKey::A => &mut self.A,
                MyEnumKey::B => &mut self.B,
                MyEnumKey::C => &mut self.C,
                MyEnumKey::D => &mut self.D,
            }
        }
    }

    impl MapValue for MyEnum {
        type Key = MyEnumKey;
        type Map = MyStruct;

        fn to_key(&self) -> Self::Key {
            match self {
                MyEnum::A => MyEnumKey::A,
                MyEnum::B(_) => MyEnumKey::B,
                MyEnum::C => MyEnumKey::C,
                MyEnum::D(_) => MyEnumKey::D,
            }
        }

        fn make_map() -> Self::Map {
            Self::Map::default()
        }
    }
}


#[test]
pub fn ensure_correct_key() {
    let value = MyEnum::A;
    let key = value.to_key();

    assert_eq!(key, MyEnumKey::A);
    assert_ne!(key, MyEnumKey::B);

    let value = MyEnum::B(10);
    let key = value.to_key();

    assert_eq!(key, MyEnumKey::B);
    assert_ne!(key, MyEnumKey::A);
}

#[test]
pub fn insert_get_map() {
    let mut m = MyEnum::make_map();

    m.insert(MyEnum::A);
    m.insert(MyEnum::B(0));
    m.insert(MyEnum::C);
    m.insert(MyEnum::D(20));
    {
        let variant_a = m.remove(&MyEnumKey::A).unwrap();
        assert_eq!(variant_a, MyEnum::A);
    }
    {
        let variant_b = m.remove(&<MyEnum as MapValue>::Key::B).unwrap();
        assert_eq!(variant_b, MyEnum::B(0))
    }
    m.insert(MyEnum::B(0));
    m.insert(MyEnum::B(10));
    {
        let variant_b = m.remove(&<MyEnum as MapValue>::Key::B).unwrap();
        assert_eq!(variant_b, MyEnum::B(10))
    }
}

#[test]
pub fn serialize() {
    let mut m = MyEnum::make_map();

    m.insert(MyEnum::A);
    m.insert(MyEnum::B(0));
    m.insert(MyEnum::C);
    m.insert(MyEnum::D(20));
    {
        println!("{:#?}", &m);
        let m_str = serde_json::to_string(&m).unwrap();
        println!("result= {:#?}", m_str);
        let expect = "[{\"D\":20},\"A\",{\"B\":0},\"C\"]";
        println!("expect~ {:#?}", expect)
    }

    m.insert(MyEnum::B(69));
    {
        println!("{:#?}", &m);
        let m_str = serde_json::to_string(&m).unwrap();
        println!("result= {:#?}", m_str);
        let expect = "[{\"D\":20},\"A\",{\"B\":69},\"C\"]";
        println!("expect~ {:#?}", expect)
    }

    let value = {
        let value = serde_json::to_value(&m).unwrap();
        let expect = Value::Array(vec![
            Value::Object({
                let mut d = SerdeMap::new();
                d.insert("D".to_string(), Value::Number(20.into()));
                d
            }),
            Value::String("A".to_string()),
            Value::Object({
                let mut b = SerdeMap::new();
                b.insert("B".to_string(), Value::Number(69.into()));
                b
            }),
            Value::String("C".to_string()),
        ]);

        println!("result= {:#?}", value);
        println!("expect~ {:#?}", expect);
        value
    };

    {
        let m2: <MyEnum as MapValue>::Map = serde_json::from_value(value).unwrap();

        let m_str = serde_json::to_string(&m).unwrap();
        let m2_str = serde_json::to_string(&m2).unwrap();

        println!("result= {:#?}", m_str);
        println!("expect~ {:#?}", m2_str);
    }
}
use variant_map::common::MapValue;
use serde_json::{Map as SerdeMap, Value};
use user::{MyEnum, MyEnumKey};

mod user {
    use variant_map::common::MapValue;
    use variant_map::hashmap::{HashKey, Map};
    use serde::{Deserialize, Serialize};
    use std::fmt::Debug;
    use std::hash::Hash;

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    pub enum MyEnum {
        A,
        C,
        B(i32),
        D(i32),
    }

    impl MapValue for MyEnum {
        type Key = MyEnumKey;
        type Map = Map<Self::Key, Self>;

        fn to_key(&self) -> Self::Key {
            match self {
                MyEnum::A => MyEnumKey::A,
                MyEnum::B(_) => MyEnumKey::B,
                MyEnum::C => MyEnumKey::C,
                MyEnum::D(_) => MyEnumKey::D,
            }
        }

        fn make_map() -> Self::Map {
            Map::default()
        }
    }

    #[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum MyEnumKey {
        // value type forces use of keys for right values
        #[serde(rename = "A")]
        A,
        #[serde(rename = "B")]
        B,
        #[serde(rename = "C")]
        C,
        #[serde(rename = "D")]
        D,
    }

    impl HashKey for MyEnumKey {}
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

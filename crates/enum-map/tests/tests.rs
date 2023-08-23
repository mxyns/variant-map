use serde_json::{Map, Value};
use crate::user::{MyEnum, MyEnumKey};
use enum_map::EnumMapValue;

mod user {
    use serde::{Deserialize, Serialize};
    use std::fmt::Debug;
    use std::hash::Hash;
    use enum_map::{EnumMap, EnumMapValue, HashKey};

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    pub enum MyEnum {
        A,
        C,
        B(i32),
        D(i32),
    }

    impl EnumMapValue for MyEnum {
        type Key = MyEnumKey;
        type Map = EnumMap<Self::Key, Self>;

        fn to_key(&self) -> Self::Key {
            match self {
                MyEnum::A => MyEnumKey::A,
                MyEnum::B(_) => MyEnumKey::B,
                MyEnum::C => MyEnumKey::C,
                MyEnum::D(_) => MyEnumKey::D,
            }
        }
    }

    impl MyEnum {
        pub fn make_map() -> <MyEnum as EnumMapValue>::Map {
            EnumMap::new()
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
fn ensure_correct_key() {
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
fn insert_get_map() {
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
        let variant_b = m.remove(&<MyEnum as EnumMapValue>::Key::B).unwrap();
        assert_eq!(variant_b, MyEnum::B(0))
    }
    m.insert(MyEnum::B(0));
    m.insert(MyEnum::B(10));
    {
        let variant_b = m.remove(&<MyEnum as EnumMapValue>::Key::B).unwrap();
        assert_eq!(variant_b, MyEnum::B(10))
    }
}


#[test]
fn serialize() {
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
        println!("{:#?}", m_str);
        let expect = "[{\"D\":20},\"A\",{\"B\":69},\"C\"]";
        println!("expect~ {:#?}", expect)
    }


    let value = {
        let value = serde_json::to_value(&m).unwrap();
        let expect = Value::Array(vec![
            Value::Object(
                {
                    let mut d = Map::new();
                    d.insert("D".to_string(), Value::Number(20.into()));
                    d
                }
            ),
            Value::String("A".to_string()),
            Value::Object(
                {
                    let mut b = Map::new();
                    b.insert("B".to_string(), Value::Number(69.into()));
                    b
                }
            ),
            Value::String("C".to_string()),
        ]);

        println!("result= {:#?}", value);
        println!("expect~ {:#?}", expect);
        value
    };

    {
        let m2: <MyEnum as EnumMapValue>::Map = serde_json::from_value(value).unwrap();

        let m_str = serde_json::to_string(&m).unwrap();
        let m2_str = serde_json::to_string(&m2).unwrap();

        println!("result= {:#?}", m_str);
        println!("expect~ {:#?}", m2_str);
    }
}

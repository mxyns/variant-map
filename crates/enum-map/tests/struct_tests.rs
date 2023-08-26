mod user {
    use serde::{Deserialize, Serialize};
    use std::fmt::Debug;
    use std::hash::Hash;

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    pub enum MyEnum {
        A,
        B(i32),
        C,
        D(i32),
    }

    pub struct MyStruct {
        __a: MyEnum,
        __b: MyEnum,
        __c: MyEnum,
        __d: MyEnum,
    }

    // TODO work on a struct flavor of enum store
    impl MyStruct {
    }
}

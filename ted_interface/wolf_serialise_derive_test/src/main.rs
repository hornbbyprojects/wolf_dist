use wolf_serialise::WolfSerialise;
#[macro_use]
use wolf_serialise_derive::WolfSerialise;

fn main() {
    println!("Hello world!");
    let test_str = TestStruct {
        foo: 3,
        bar: Vec::new(),
    };
    let test_enum = TestEnum::NewType(test_str.clone(), test_str.clone());
    match test_enum {
        TestEnum::NewType(_inner_0, _inner_1) => {}
        _ => panic!("Oh no!"),
    }
}

#[derive(WolfSerialise, PartialEq, Eq, Debug, Clone)]
struct TestStruct {
    foo: i32,
    bar: Vec<i32>,
}

#[derive(WolfSerialise, PartialEq, Eq, Debug, Clone)]
struct NestedStruct {
    foo: i32,
    bar: Vec<TestStruct>,
}

#[derive(PartialEq, Eq, Debug, Clone, WolfSerialise)]
enum TestEnum {
    NewType(TestStruct, TestStruct),
    Unit,
}

#[macro_use]
extern crate wolf_serialise_derive;

use wolf_serialise::WolfSerialise;

#[derive(WolfSerialise, PartialEq, Eq, Debug)]
struct TestStruct {
    foo: i32,
    bar: Vec<i32>,
}

#[derive(WolfSerialise, PartialEq, Eq, Debug)]
struct NestedStruct {
    foo: i32,
    bar: Vec<TestStruct>,
}

fn test_serialise<T: WolfSerialise + Eq + std::fmt::Debug>(input: T) {
    let buffer = Vec::new();
    let mut cursor = std::io::Cursor::new(buffer);
    input
        .wolf_serialise(&mut cursor)
        .expect("error serialising");
    let serialised = cursor.into_inner();
    let deserialised =
        T::wolf_deserialise(&mut std::io::Cursor::new(serialised)).expect("error deserialising");
    assert_eq!(input, deserialised)
}

#[test]
fn test_works() {
    let test_str = TestStruct {
        foo: 2,
        bar: vec![2, 1, 3],
    };
    test_serialise(test_str);
}

#[test]
fn test_works_nested() {
    let test_str_1 = TestStruct {
        foo: 2,
        bar: vec![2, 1, 3],
    };
    let test_str_2 = TestStruct {
        foo: 1,
        bar: vec![5, 1, 3],
    };
    let nested_str = NestedStruct {
        foo: 5,
        bar: vec![test_str_2, test_str_1],
    };
    test_serialise(nested_str);
}

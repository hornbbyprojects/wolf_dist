#[macro_use]
extern crate wolf_serialise_derive;

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

#[derive(WolfSerialise, PartialEq, Eq, Debug)]
enum TestEnum {
    NewType(TestStruct, TestStruct),
    Unit,
}

#[test]
fn test_works() {
    let test_str_1 = TestStruct {
        foo: 2,
        bar: vec![2, 1, 3],
    };
    let test_str_2 = TestStruct {
        foo: 2,
        bar: vec![5, 1, 1],
    };
    let test_enum = TestEnum::NewType(test_str_1, test_str_2);
    test_serialise(test_enum);
}

#[test]
fn test_newtype_size() {
    let test_str_1 = TestStruct {
        foo: 2,
        bar: vec![2, 1, 3],
    };
    let test_str_2 = TestStruct {
        foo: 2,
        bar: vec![6, 7, -1],
    };
    let test_enum = TestEnum::NewType(test_str_1, test_str_2);
    let buffer = Vec::new();
    let mut cursor = std::io::Cursor::new(buffer);
    test_enum.wolf_serialise(&mut cursor).unwrap();
    let output = cursor.into_inner();
    // 1 enum discriminant + 2*(4 foo + 4 bar size + 4*3 bar contents)
    assert_eq!(output.len(), 1 + 2 * (4 + 4 + 4 * 3));
}

#[test]
fn test_error_on_bad_discriminator() {
    let buffer = vec![2];
    let mut cursor = std::io::Cursor::new(buffer);
    match TestEnum::wolf_deserialise(&mut cursor) {
        Ok(_) => panic!("Didn't return error on invalid discriminator"),
        Err(e) => {
            assert_eq!(e.to_string(), "2 is not a valid discriminator for TestEnum")
        }
    }
}

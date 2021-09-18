use super::*;
use assert2::assert;

#[test]
fn on_empty_container_builder_increases_len_to_1() {
    // Given an empty list
    let item = 42_i32;
    let expected_len = 1;
    let sut = ContainerBuilder::new();
    assert!(sut.is_empty());

    // When
    let result = sut.register_instance(item);

    // Then
    assert!(result.len() == expected_len);
}

#[test]
fn on_container_with_len_1_increases_len_to_2() {
    // Given an list containing one item
    let given_item = 42_i32;
    let item = Foo { _bar: -7, _baz: "test" };
    let expected_len = 2;
    let sut = ContainerBuilder::new().register_instance(given_item);
    assert!(sut.len() == 1);

    // And When
    let result = sut.register_instance(item);

    // Then
    assert!(result.len() == expected_len);
}

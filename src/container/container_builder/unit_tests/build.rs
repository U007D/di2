mod identify_type;

use super::*;
use assert2::assert;
use identify_type::is_type_container;

#[test]
fn constructs_expected_container() {
    // Given
    let item_1 = Foo { _bar: -7, _baz: "test" };
    let item_2 = String::from("Hello, world!");
    let expected_len = 2;
    let sut = ContainerBuilder::new().register_instance(item_1).register_instance(item_2);

    // When
    let result = sut.build();

    // Then
    assert!(is_type_container(&result));

    // And then
    assert!(result.len() == expected_len);
}

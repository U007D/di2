use super::*;
use assert2::assert;

#[test]
fn builds_empty_container_builder() {
    // When
    let result = ContainerBuilder::new();

    // Then
    assert!(result.is_empty());
}

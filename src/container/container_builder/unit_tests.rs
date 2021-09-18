#![allow(clippy::unwrap_used)]
mod build;
mod new;
mod register_instance;

use super::*;

// Arbitary custom type used in some unit tests
#[derive(Debug)]
struct Foo {
    _bar: i8,
    _baz: &'static str,
}

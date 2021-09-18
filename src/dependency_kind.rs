use std::{
    fmt::Debug,
    any::TypeId
};

#[derive(Debug)]
pub enum DependencyKind<T> {
    Instance(T),
    Type(TypeId),
}

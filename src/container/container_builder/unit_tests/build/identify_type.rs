use super::*;
use std::any::Any;

pub trait IdentifyType: Any {
    fn r#type(&self) -> Type;
}

#[derive(Debug, PartialEq, Eq)]
pub enum Type {
    Container,
    Unidentified,
}

impl<T: Any> IdentifyType for Container<T> {
    fn r#type(&self) -> Type {
        Type::Container
    }
}

impl IdentifyType for dyn Any {
    fn r#type(&self) -> Type {
        Type::Unidentified
    }
}

pub fn is_type_container<T: IdentifyType>(item: &T) -> bool {
    item.r#type() == Type::Container
}

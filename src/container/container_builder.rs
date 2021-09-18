#[cfg(test)]
mod unit_tests;

use crate::container::Container;
use crate::dependency_kind::DependencyKind;
use frunk::{hlist::HList as IHList, HCons, HNil};
use std::fmt::Debug;

#[derive(Debug, Default)]
pub struct ContainerBuilder<THList>(THList);

impl ContainerBuilder<HNil> {
    // Constructors
    pub const fn new() -> Self {
        Self(HNil)
    }
}

impl<THList> ContainerBuilder<THList>
where
    THList: IHList,
{
    // Public methods
    pub fn build(self) -> Container<THList> {
        Container::with(self.0)
    }

    pub fn register_instance<TItem>(
        self,
        item: TItem,
    ) -> ContainerBuilder<HCons<DependencyKind<TItem>, THList>> {
        ContainerBuilder(self.0.prepend(DependencyKind::Instance(item)))
    }

    // Private methods
    #[allow(dead_code)]
    fn is_empty(&self) -> bool { self.len() == 0 }

    #[allow(clippy::unused_self, dead_code)]
    fn len(&self) -> usize {
        <THList as IHList>::LEN
    }
}

mod container_builder;

use frunk::hlist::{HList as IHList, Plucker as IPlucker};

#[derive(Debug)]
pub struct Container<THList>(THList);

impl<THList> Container<THList>
where
    THList: IHList,
{
    // Constructors
    fn with(items: THList) -> Self {
        Self(items)
    }

    // Public methods
    pub fn resolve<T, TIndex>(self) -> (T, Container<THList::Remainder>)
    where
        THList: IPlucker<T, TIndex>,
    {
        let (item, remainder) = self.0.pluck();
        (item, Container(remainder))
    }

    // Private methods
    #[allow(dead_code)]
    fn is_empty(&self) -> bool { self.len() == 0 }

    #[allow(clippy::unused_self, dead_code)]
    fn len(&self) -> usize {
        <THList as IHList>::LEN
    }
}

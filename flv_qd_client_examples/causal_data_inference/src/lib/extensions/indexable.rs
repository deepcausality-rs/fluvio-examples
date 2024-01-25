use crate::prelude::TimeIndexable;
use crate::protocols::indexable::Indexable;
use crate::types::alias::CustomContext;

impl Indexable for CustomContext<'_> {
    fn get_index(&self, key: usize, current: bool) -> usize {

        todo!()
    }

    fn set_index(&mut self, key: usize, index: usize, current: bool) {
        todo!()
    }
}

impl TimeIndexable for CustomContext<'_> {}

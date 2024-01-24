use crate::prelude::Indexable;
use crate::types::alias::CustomContext;

impl Indexable for CustomContext<'_> {
    fn get_current_year_index(&self) -> usize {
        todo!()
    }

    fn get_current_month_index(&self) -> usize {
        todo!()
    }

    fn set_current_year_index(&mut self, _index: usize) {
        todo!()
    }

    fn set_current_month_index(&mut self, _index: usize) {
        todo!()
    }

    fn get_previous_year_index(&self) -> usize {
        todo!()
    }

    fn get_previous_month_index(&self) -> usize {
        todo!()
    }

    fn set_previous_year_index(&mut self, _index: usize) {
        todo!()
    }

    fn set_previous_month_index(&mut self, _index: usize) {
        todo!()
    }
}

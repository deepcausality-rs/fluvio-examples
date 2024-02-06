use causal_model::prelude::{CustomContext, TimeIndexExt};
use deep_causality::prelude::Context;

fn get_indexable_context() -> CustomContext<'static> {
    Context::with_capacity(1, "Causal Context", 10)
}

#[test]
fn test_get_current_year_index() {
    let mut indexable = get_indexable_context();
    assert_eq!(indexable.get_current_year_index(), None);

    indexable.set_current_year_index(2022);
    assert_eq!(indexable.get_current_year_index(), Some(&2022));
}

#[test]
fn test_get_current_month_index() {
    let mut indexable = get_indexable_context();
    assert_eq!(indexable.get_current_month_index(), None);

    indexable.set_current_month_index(6);
    assert_eq!(indexable.get_current_month_index(), Some(&6));
}

#[test]
fn test_get_previous_year_index() {
    let mut indexable = get_indexable_context();
    assert_eq!(indexable.get_previous_year_index(), None);

    indexable.set_previous_year_index(2021);
    assert_eq!(indexable.get_previous_year_index(), Some(&2021));
}

#[test]
fn test_get_previous_month_index() {
    let mut indexable = get_indexable_context();
    assert_eq!(indexable.get_previous_month_index(), None);

    indexable.set_previous_month_index(5);
    assert_eq!(indexable.get_previous_month_index(), Some(&5));
}

use causal_model::model::build_main_causaloid::build_main_causaloid;
use causal_model::prelude::CustomContext;
use deep_causality::prelude::{CausableGraph, Identifiable};

#[test]
fn test_build_main_causaloid() {
    let context = CustomContext::with_capacity(0, "test_build_causal_model", 10);

    let causaloid = build_main_causaloid(&context);

    assert_eq!(causaloid.id(), 0);
    assert_eq!(causaloid.description(), "Causaloid main graph");

    let has_root = causaloid.causal_graph().unwrap().contains_root_causaloid();
    assert!(has_root);

    let has_month = causaloid.causal_graph().unwrap().contains_causaloid(1);
    assert!(has_month);
}

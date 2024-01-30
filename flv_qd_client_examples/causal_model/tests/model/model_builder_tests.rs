use deep_causality::prelude::Identifiable;
use causal_model::model::causaloid_range_month;
use causal_model::prelude::{CustomContext};
use causal_model::prelude::model::model_builder;

#[test]
fn test_build_causal_model() {
    let context = CustomContext::with_capacity(0, "test_build_causal_model", 10);
    let causaloid = causaloid_range_month::get_month_causaloid(&context, 1);

    let model = model_builder::build_causal_model(&context, causaloid);

    assert_eq!(model.id(), 1);
    assert_eq!(model.author(), &"Marvin Hansen <marvin.hansen@gmail.com>");
    assert_eq!(model.description(), &"This is a test causal model with context");
    assert!(model.assumptions().is_none());
}
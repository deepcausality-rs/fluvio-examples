use causal_model::prelude::model::model_builder;
use causal_model::prelude::CustomContext;
use deep_causality::prelude::Identifiable;

#[test]
fn test_build_causal_model() {
    let context = CustomContext::with_capacity(0, "test_build_causal_model", 10);

    let model = model_builder::build_causal_model(&context);

    assert_eq!(model.id(), 42);
    assert_eq!(
        model.description(),
        "Causal Model: Checks for a potential monthly long breakout"
    );
}

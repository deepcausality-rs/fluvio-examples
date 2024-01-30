use crate::prelude::{CustomCausaloid, CustomContext, CustomModel};
use deep_causality::prelude::Model;

/// Builds a custom [`Model`] from a context graph and causaloid.
///
/// Constructs a new [`Model`] with the provided context graph,
/// causaloid, author, description, etc.
///
/// The built model contains the full context graph and causaloid
/// representing a causal model.
///
/// # Arguments
///
/// * `context` - Context graph to include in the model
/// * `causaloid` - Causaloid to include in the model
///
/// # Returns
///
/// The built [`CustomModel`] containing the provided context and causaloid.
///
pub fn build_causal_model<'l>(
    context: &'l CustomContext<'l>,
    causaloid: CustomCausaloid<'l>,
) -> CustomModel<'l> {
    let id = 1;
    let author = "Marvin Hansen <marvin.hansen@gmail.com>";
    let description = "This is a test causal model with context";
    let assumptions = None;

    Model::new(
        id,
        author,
        description,
        assumptions,
        causaloid,
        Some(context),
    )
}

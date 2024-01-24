use deep_causality::prelude::{
    CausalityError, Causaloid, Contextuable, ContextuableGraph, IdentificationValue, NumericalValue,
};
use rust_decimal::prelude::ToPrimitive;

use crate::prelude::{Indexable, Rangeable};
use crate::types::alias::{CustomCausaloid, CustomContext};

/// Creates a new Causaloid that checks if the current price exceeds the previous day's range high level.
///
/// # Arguments
///
/// * `context` - The context containing the previous day's data
/// * `id` - The ID to assign to the Causaloid
///
/// # Returns
///
/// A new CustomCausaloid instance.
///
/// # Functionality
///
/// The causal function takes the current price and checks if it exceeds the
/// previous day's range high level from the context. It returns a boolean indicating if this condition
/// is met.
///
/// The cause is the price exceeding the range high, and the effect is a potential breakout.
///
/// The function uses closures to break the logic into smaller reusable parts:
///
/// - Checking the day open/close range direction
/// - Checking if price exceeds the high
///
/// This allows the main logic to be straightforward.
///
/// The context is used to lookup the previous day's data via the Indexable trait.
///
pub(crate) fn get_year_causaloid<'l>(
    context: &'l CustomContext<'l>,
    id: IdentificationValue,
) -> CustomCausaloid<'l> {
    let description =
        "Checks if the current price exceeds the range high level of the previous day.";

    // The causal function is a function that takes the current price and returns a boolean
    // that indicates whether the current price exceeds the monthly high level.
    // The cause being some fabricated nonsense metrics i.e. price above monthly high and the effect
    // being a monthly breakout.

    // The causal fucntion must be a function and not a closure because the function
    // will be coercived into a function pointer later on, which is not possible with a closure.
    // Within the causal function, you can write safely as many closures as you want. See below.
    fn contextual_causal_fn<'l>(
        obs: NumericalValue,
        ctx: &'l CustomContext<'l>,
    ) -> Result<bool, CausalityError> {
        if obs.is_nan() {
            return Err(CausalityError("Observation is NULL/NAN".into()));
        }

        // We use a dynamic secondary index to determine the actual index of the previous or current day tempoid relative
        // to the now() timestamp. To do this, we  extend the context with an extension trait and corresponding implementation.
        // See http://xion.io/post/code/rust-extension-traits.html
        let year = ctx
            .get_node(ctx.get_current_year_index())
            .expect("node for current month not found");

        // Get the range of the current year.
        let year_range = year
            .vertex_type()
            .dataoid()
            .expect("Failed to get data out of year node");

        // closure that captures the context within the causal function.
        let check_price_above_year_open =
            || obs.gt(&year_range.data_range().open().to_f64().unwrap());

        // With the closures in place, the main logic becomes straightforward and simple to understand.
        if check_price_above_year_open() {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    Causaloid::new_with_context(id, contextual_causal_fn, Some(context), description)
}

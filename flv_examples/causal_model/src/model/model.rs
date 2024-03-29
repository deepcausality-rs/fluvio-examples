use crate::prelude::{context_utils, CustomCausaloid, CustomContext, TimeIndexExt};
use deep_causality::errors::CausalityError;
use deep_causality::prelude::{Causaloid, NumericalValue};
use rust_decimal::Decimal;

/// Builds a custom [`Causaloid`] from a context graph.
///
/// Constructs a new [`Causaloid`] with the provided context graph,
/// causaloid, author, description, etc.
///
/// The built model contains the full context graph and causaloid
/// representing a causal model.
///
/// # Arguments
///
/// * `context` - Context graph to include in the model
///
/// # Returns
///
/// The built [`Causaloid`] .
///
pub fn build_causal_model<'l>(context: &'l CustomContext<'l>) -> CustomCausaloid<'l> {
    let id = 42;
    // The causal fucntion must be a function and not a closure because the function
    // will be coercived into a function pointer later on, which is not possible with a closure.
    let contextual_causal_fn = contextual_causal_function;
    let description = "Causal Model: Checks for a potential monthly long breakout";

    // Constructs and returns the Causaloid.
    Causaloid::new_with_context(id, contextual_causal_fn, Some(context), description)
}

/// Creates a new causal function that checks if the current price exceeds the previously established monthly high price.
///
/// The monthly breakout is defined as the following price action:
///
/// 1) Check if the previous month close is above the previous month open.
/// 2) Check if the current price is above the previous months close price.
/// 3) Check if the current price is above the current month open price.
/// 4) Check if the current price exceeds the high level of the previous month.
///
/// If, and only if all conditions are true, then a monthly breakout is detected.
///
/// # Arguments
/// * obs - The observation to evaluate
/// * `context` - The context of the instrument. This is used to lookup the current and previous month data nodes.
///
/// # Returns true if the causal relation has been observed, that is, if the current price is a potential monthly breakout.
///
///
/// # Functionality
///
/// The purpose is to create a causal function that checks for a potential monthly breakout.
///
/// 1. It defines the causal function that will check for the monthly breakout condition.
/// 2. The causal function takes the price observation and context as arguments.
/// 3. It uses the context to lookup the current and previous month data node.
/// 4. The data is extracted from the node.
/// 5. The price observation is compared to determine a potential monthly breakout.
/// 6. A boolean is returned indicating if the price is a potential monthly breakout.
/// 7. The causal function returns a new Causaloid::new_with_context.
///
/// This allows to detect potential monthly breakouts in a simple way.
/// The context is used to dynamically look up the required data nodes.
///
fn contextual_causal_function<'l>(
    obs: NumericalValue,
    ctx: &'l CustomContext<'l>,
) -> Result<bool, CausalityError> {
    // Check if current_price data is available, if not, return an error.
    if obs.is_nan() {
        return Err(CausalityError(
            "Month Causaloid: Observation/current_price is NULL/NAN".into(),
        ));
    }

    // Convert f64 to Decimal to avoid precision loss and make the code below more readable.
    // Unwrap is safe because of the previous null check, we know that the current price is not null.
    let current_price = Decimal::from_f64_retain(obs).unwrap();

    // We use a dynamic index to determine the actual index of the previous or current month.
    // Unwrap is safe here because the build_context function ensures that the current month is always initialized with a valid value.
    let current_month_index = *ctx.get_current_month_index().unwrap();
    let previous_month_index = *ctx.get_previous_month_index().unwrap();

    // We use the dynamic index to extract the RangeData from the current and previous month.
    let current_month_data = context_utils::extract_data_from_ctx(ctx, current_month_index)?;
    let previous_month_data = context_utils::extract_data_from_ctx(ctx, previous_month_index)?;

    // The logic below is obviously totally trivial, but it demonstrates that you can
    // easily split an arbitrary complex causal function into multiple closures.
    // With closures in place, the logic becomes straightforward, robust, and simple to understand.

    // 1) Check if the previous month close is above the previous month open.
    let check_previous_month_close_above_previous_open = || {
        // Test if the previous month close is above the previous month open.
        // This is indicative of a general uptrend and gives a subsequent breakout more credibility.
        previous_month_data.close_above_open()
    };

    // 2) Check if the current price is above the previous months close price.
    let check_current_price_above_previous_close = || {
        // Test if the current price is above the previous months close price.
        // gt = greater than > operator
        current_price.gt(&previous_month_data.close())
    };

    // 3) Check if the current price is above the current month open price.
    // This may seem redundant, but it safeguards against false positives.
    let check_current_price_above_current_open = || {
        // Test if the current price is above the current month open price.
        current_price.gt(&current_month_data.open())
    };

    // 4) Check if the current price exceeds the high level of the previous month.
    let check_current_price_above_previous_high = || {
        // Test if the current price is above the high price established in the previous month.
        current_price.gt(&previous_month_data.high())
    };

    // All checks combined:
    //
    // 1) Check if the previous month close is above the previous month open.
    // 2) Check if the current price is above the previous months close price.
    // 3) Check if the current price is above the current month open price.
    // 4) Check if the current price exceeds the high level of the previous month.
    if check_previous_month_close_above_previous_open()
        && check_current_price_above_previous_close()
        && check_current_price_above_current_open()
        && check_current_price_above_previous_high()
    {
        // If all conditions are true, then a monthly breakout is detected and return true.
        Ok(true)
    } else {
        // If any of the conditions are false, then no breakout is detected and return false.
        Ok(false)
    }
}

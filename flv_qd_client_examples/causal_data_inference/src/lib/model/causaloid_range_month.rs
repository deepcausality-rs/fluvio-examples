use crate::prelude::{context_utils, TimeIndexable};
use crate::types::alias::{CustomCausaloid, CustomContext};
use deep_causality::prelude::{CausalityError, Causaloid, IdentificationValue, NumericalValue};
use rust_decimal::Decimal;

/// Creates a new Causaloid that checks if the current price exceeds the previously established monthly high price.
///
/// The monthly breakout is defined as the following price action:
///
/// 0) if the previous month close is above the previous month open.
/// 1) if the current spot price exceeds the high level of the previous month,
/// 2) if the current spot price is above the previous months close price,
/// 3) if the current spot price is above the current month open price,
///
/// If, and only if, all conditions are true, then a monthly breakout is detected.
///
/// # Arguments
///
/// * `context` - The context containing the current month data
/// * `id` - The ID to assign to the Causaloid
///
/// # Returns
///
/// A new CustomCausaloid instance.
///
/// # Functionality
///
/// The purpose is to create a Causaloid that checks for a potential monthly breakout, that is,
///  when the current price exceeds the previous highest level for the month.
///
/// 1. It defines the causal function that will check for the monthly breakout condition.
/// 2. The causal function takes the price observation and context as arguments.
/// 3. It uses the context to lookup the current and previous month data node.
/// 4. The data is extracted from the node.
/// 5. The price observation is compared to determine a potential monthly breakout.
/// 6. A boolean is returned indicating if the price exceeds the monthly high.
/// 7. The causal function is passed to Causaloid::new_with_context to create the Causaloid.
///
/// This allows creating a Causaloid to detect potential monthly breakouts in a simple way.
/// The context handles looking up the required data dynamically.
///
pub(crate) fn get_month_causaloid<'l>(
    context: &'l CustomContext<'l>,
    id: IdentificationValue,
) -> CustomCausaloid<'l> {
    //
    let description = "Checks for a potential monthly long breakout";

    // The causal fucntion must be a function and not a closure because the function
    // will be coercived into a function pointer later on, which is not possible with a closure.
    // Within the causal function, you can write safely as many closures as you want. See below.
    fn contextual_causal_fn<'l>(
        obs: NumericalValue,
        ctx: &'l CustomContext<'l>,
    ) -> Result<bool, CausalityError> {
        // Check if current_price data is available, if not, return an error.
        if obs.is_nan() {
            return Err(CausalityError(
                "Observation/current_price is NULL/NAN".into(),
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

        // Check if the previous month close is above the previous month open.
        let check_previous_month_close_above_open = || {
            // Test if the previous month close is above the previous month open.
            // This is indicative of a general uptrend and gives a subsequent breakout more credibility.
            previous_month_data.close_above_open()
        };

        // Check if the current price is above the previous months close price.
        let check_current_price_above_previous_close = || {
            // Test if the current price is above the previous months close price.
            // gt = greater than > operator
            current_price.gt(&previous_month_data.close())
        };

        // Check if the current price is above the current month open price.
        // This may seem redundant, but it safeguards against false positives.
        let check_current_price_above_current_open = || {
            // Test if the current price is above the current month open price.
            current_price.gt(&current_month_data.open())
        };

        // Check if the current price exceeds the high level of the previous month.
        let check_current_price_above_previous_high = || {
            // Test if the (current price) is above the current high price of the current month.
            current_price.gt(&previous_month_data.high())
        };

        // Check if the current price exceeds the high level of the previous month,
        // and if the current price is above the previous months close price,
        // and if the previous month close is above the previous month open.
        // If all conditions are true, then a monthly breakout is detected and returns true.
        //
        // Note, you can do arbitrary complex control flow as long as its deterministic.
        if check_current_price_above_previous_high()
            && check_current_price_above_previous_close()
            && check_current_price_above_current_open()
            && check_previous_month_close_above_open()
        {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    // Constructs and returns the Causaloid.
    Causaloid::new_with_context(id, contextual_causal_fn, Some(context), description)
}

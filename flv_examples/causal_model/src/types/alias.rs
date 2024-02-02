use deep_causality::prelude::{BaseNumberType, Causaloid, Context, Model, Space, SpaceTime, Time};

use crate::types::range_data::RangeData;

/// Custom context graph is a type alias for a Context generic over the custom types:
///
/// - `RangeData`: Custom data type
/// - `Space<BaseNumberType>`: Space dimension of base number types
/// - `Time<BaseNumberType>`: Time series of base number types
/// - `SpaceTime<BaseNumberType>`: Spacetime of base number types
/// - BaseNumberType: The base number type
///
///
/// Custom context extends the base Context graph and adds the following functionality:
///
/// - get_current_year_index() - Get index of current year node.
/// - set_current_year_index() - Set current year node index.
/// - get_previous_year_index() - Get index of previous year node.
/// - set_previous_year_index() - Set previous year node index.
/// - get_current_month_index() - Get index of current month node.
/// - set_current_month_index() - Set current month node index.
/// - get_previous_month_index() - Get index of previous month node.
/// - set_previous_month_index() - Set previous month node index.
///
/// These methods are defined in the inedexable trait and added as a type extension.
/// See the following files for details:
///
/// - flv_examples/causal_data_inference/src/lib/protocols/time_indexable
/// - flv_examples/causal_data_inference/src/lib/extensions/time_indexable
///
/// This provides easy access to the special nodes in the hierarchy
/// that track the current and previous year and month.
///
pub type CustomContext<'l> = Context<
    'l,
    RangeData,
    Space<BaseNumberType>,
    Time<BaseNumberType>,
    SpaceTime<BaseNumberType>,
    BaseNumberType,
>;

/// CustomCausaloid is a type alias for a Causaloid generic over the custom types:
///
/// - `RangeData`: Custom data type
/// - `Space<BaseNumberType>`: Space dimension of base number types
/// - `Time<BaseNumberType>`: Time series of base number types
/// - `SpaceTime<BaseNumberType>`: Spacetime of base number types
/// - BaseNumberType: The base number type
///
/// This provides a convenient way to create a Causaloid for our custom types.
///
pub type CustomCausaloid<'l> = Causaloid<
    'l,
    RangeData,
    Space<BaseNumberType>,
    Time<BaseNumberType>,
    SpaceTime<BaseNumberType>,
    BaseNumberType,
>;

/// CustomModel is a type alias for a Model generic over the custom types:
///
/// - `RangeData`: Custom data type
/// - `Space<BaseNumberType>`: Space dimension of base number types
/// - `Time<BaseNumberType>`: Time dimension of base number types
/// - `SpaceTime<BaseNumberType>`: Spacetime of base number types
/// - BaseNumberType: The base number type
///
/// This provides a convenient way to create a Model for our custom types.
///
pub type CustomModel<'l> = Model<
    'l,
    RangeData,
    Space<BaseNumberType>,
    Time<BaseNumberType>,
    SpaceTime<BaseNumberType>,
    BaseNumberType,
>;

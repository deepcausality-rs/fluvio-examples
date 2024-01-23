use deep_causality::prelude::{BaseNumberType, Causaloid, Context, Model, Space, SpaceTime, Time};

use crate::types::range_data::RangeData;

/// CustomContext is a type alias for a Context generic over the custom types:
///
/// - CustomData: Our custom data type
/// - Space<BaseNumberType>: Vector space of base number types
/// - Time<BaseNumberType>: Time series of base number types
/// - SpaceTime<BaseNumberType>: Spacetime of base number types
/// - BaseNumberType: The base number type
///
/// This provides a convenient way to create a Context for our custom types.
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
/// - CustomData: Our custom data type
/// - Space<BaseNumberType>: Vector space of base number types
/// - Time<BaseNumberType>: Time series of base number types
/// - SpaceTime<BaseNumberType>: Spacetime of base number types
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
/// - CustomData: Our custom data type
/// - Space<BaseNumberType>: Vector space of base number types
/// - Time<BaseNumberType>: Time series of base number types
/// - SpaceTime<BaseNumberType>: Spacetime of base number types
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

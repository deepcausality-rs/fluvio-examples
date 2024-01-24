use common::prelude::TimeScale;


/// Returns a boolean control map for aggregating time series data
/// based on the provided [`TimeScale`].
///
/// The returned vector contains a boolean value for each time unit,
/// indicating whether data should be aggregated for that unit based
/// on the given [`TimeScale`]. The time units in order are:
///
/// 0: Year
/// 1: Quarter
/// 2: Month
/// 3: Week
/// 4: Day
/// 5: Hour
/// 6: Minute
/// 7: Second
///
/// For example, if [`TimeScale::Day`] is passed, the returned vector
/// would be [true, true, true, true, false, false, false, false],
/// indicating aggregation should happen for Year, Quarter, Month,
/// and Week, but not for Day, Hour, Minute, or Second.
///
/// # Arguments
///
/// * `time_scale` - The [`TimeScale`] to generate the control map for.
///
/// # Returns
///
/// A `Vec<bool>` containing the aggregation control map for the given `TimeScale`.
///
pub fn get_boolean_control_map(time_scale: TimeScale) -> Vec<bool> {
    match time_scale {
        // Boolean Index:
        // 0: Year,1: Quarter,2: Month,3: Week,4: Day,5: Hour,6: Minute, 7: Second
        TimeScale::NoScale => vec![true, true, true, true, true, true, true, true],
        TimeScale::Second => vec![true, true, true, true, true, true, true, true],
        TimeScale::Minute => vec![true, true, true, true, true, true, true, false],
        TimeScale::Hour => vec![true, true, true, true, true, true, false, false],
        TimeScale::Day => vec![true, true, true, true, true, false, false, false],
        TimeScale::Week => vec![true, true, true, true, false, false, false, false],
        TimeScale::Month => vec![true, true, true, false, false, false, false, false],
        TimeScale::Quarter => vec![true, true, false, false, false, false, false, false],
        TimeScale::Year => vec![true, false, false, false, false, false, false, false],
    }
}
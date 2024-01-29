use chrono::{Datelike, Timelike};
use common::prelude::OHLCVBar;
use deep_causality::prelude::TimeScale;

/// Returns a boolean control map for aggregating time series data
/// based on the provided [`TimeScale`].
///
/// The returned vector contains a boolean value for each time unit,
/// indicating whether data should be aggregated for that unit based
/// on the given [`TimeScale`].
///
/// The time units in order are:
///
/// - Year
/// - Quarter
/// - Month
/// - Week
/// - Day
/// - Hour
/// - Minute
/// - Second
///
/// # Arguments
///
/// * `time_scale` - The [`TimeScale`] to generate the control map for.
///
/// # Returns
///
/// A `Vec<bool>` containing the aggregation control map for the given
/// `TimeScale`.
///
/// # Examples
///
/// ```
/// use deep_causality::prelude::TimeScale;
/// use causal_model::prelude::time_utils::get_time_scale_control_map;
///
/// let hour_map = get_time_scale_control_map(&TimeScale::Hour);
/// // [true, true, true, true, true, false, false, false]
/// ```
///
pub fn get_time_scale_control_map(time_scale: &TimeScale) -> Vec<bool> {
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

/// Returns the time unit value for an [`OHLCVBar`] based on the provided [`TimeScale`].
///
/// Gets the appropriate time field from the [`OHLCVBar`]'s DateTime field
/// based on the desired [`TimeScale`] aggregation.
///
/// For example if [`TimeScale::Hour`] is passed, this will return the
/// hour field of the bar's DateTime field
///
/// # Arguments
///
/// * `data_bar` - The [`OHLCVBar`] to get the time unit value for.
/// * `time_scale` - The [`TimeScale`] indicating which time unit to return.
///
/// # Returns
///
/// The u32 time unit value for the bar based on the provided [`TimeScale`].
///
pub fn get_time_unit(data_bar: &OHLCVBar, time_scale: TimeScale) -> u32 {
    match time_scale {
        TimeScale::NoScale => data_bar.date_time().minute(),
        TimeScale::Second => data_bar.date_time().second(),
        TimeScale::Minute => data_bar.date_time().minute(),
        TimeScale::Hour => data_bar.date_time().hour(),
        TimeScale::Day => data_bar.date_time().day(),
        TimeScale::Week => data_bar.date_time().iso_week().week(),
        TimeScale::Month => data_bar.date_time().month(),
        TimeScale::Quarter => data_bar.date_time().year() as u32,
        TimeScale::Year => data_bar.date_time().year() as u32,
    }
}

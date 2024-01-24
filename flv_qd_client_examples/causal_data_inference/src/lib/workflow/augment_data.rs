use crate::prelude::time_utils;
use crate::prelude::{BarRange, RangeData};
use common::prelude::OHLCVBar;
use deep_causality::prelude::{BaseNumberType, Time, TimeScale};

/// Converts an [`OHLCVBar`] to a temporally augmented [`RangeData`] object.
///
/// Gets the timestamp, time unit, and range data from the provided
/// [`OHLCVBar`]. Constructs a [`Time`] and [`RangeData`] object
/// representing the bar data in a temporally augmented format.
///
/// # Arguments
///
/// * `data_bar` - The [`OHLCVBar`] to convert.
/// * `time_scale` - The [`TimeScale`] to use for temporal augmentation.
///
/// # Returns
///
/// A tuple containing the constructed [`Time`] and [`RangeData`] objects
/// representing the augmented [`OHLCVBar`] data.
///
pub fn convert_ohlcv_bar_to_augmented(
    data_bar: &OHLCVBar,
    time_scale: TimeScale,
) -> (Time<BaseNumberType>, RangeData) {
    let id = data_bar.date_time().timestamp() as u64;
    let data_range = calculate_ohlcv_ranges(data_bar);

    let time_unit = time_utils::get_time_unit(data_bar, time_scale) as BaseNumberType;
    let tempoid = Time::new(id, time_scale, time_unit);
    let dataoid = RangeData::new(id, data_range);

    (tempoid, dataoid)
}

/// Calculates the range data from an [`OHLCVBar`].
///
/// Gets the high, close, and whether the close is above/below the open
/// from the provided [`OHLCVBar`].
///
/// # Arguments
///
/// * `data_bar` - The [`OHLCVBar`] to calculate range data for.
///
/// # Returns
///
/// A [`BarRange`] containing the range data for the bar.
///
fn calculate_ohlcv_ranges(data_bar: &OHLCVBar) -> BarRange {
    let high = data_bar.high();
    let close = data_bar.open();
    let close_above_open = data_bar.close() > data_bar.open();
    let close_below_open = data_bar.close() < data_bar.open();

    BarRange::new(high, close, close_above_open, close_below_open)
}

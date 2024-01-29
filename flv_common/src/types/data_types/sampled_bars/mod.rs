use crate::prelude::OHLCVBar;

/// Holds OHLCV bars aggregated at different time scales.
///
/// Contains separate fields for daily, monthly, and yearly
/// OHLCV bars. This allows accessing bars that have been
/// aggregated to different time units from the same struct.
///
/// # Fields
///
/// * `day_bars` - Vector of [`OHLCVBar`] aggregated to daily values.
/// * `month_bars` - Vector of [`OHLCVBar`] aggregated to monthly values.
/// * `year_bars` - Vector of [`OHLCVBar`] aggregated to yearly values.
///
/// # Methods
///
/// * `new` - Constructs a new empty [`SampledDataBars`].
/// * `set_day_bars` - Sets the daily [`OHLCVBar`] vector.
/// * `set_month_bars` - Sets the monthly [`OHLCVBar`] vector.
/// * `set_year_bars` - Sets the yearly [`OHLCVBar`] vector.
/// * `day_bars` - Gets a reference to the daily [`OHLCVBar`] vector.
/// * `month_bars` - Gets a reference to the monthly [`OHLCVBar`] vector.
/// * `year_bars` - Gets a reference to the yearly [`OHLCVBar`] vector.
///
#[derive(Debug, Default, Clone)]
pub struct SampledDataBars {
    day_bars: Vec<OHLCVBar>,
    month_bars: Vec<OHLCVBar>,
    year_bars: Vec<OHLCVBar>,
}

/// Constructs a new empty [`SampledDataBars`].
///
/// Initializes the `day_bars`, `month_bars`, and `year_bars` fields
/// to empty vectors.
///
/// # Returns
///
/// A new [`SampledDataBars`] instance with empty aggregrated bar vectors.
///
impl SampledDataBars {
    pub fn new() -> Self {
        Self {
            day_bars: Vec::new(),
            month_bars: Vec::new(),
            year_bars: Vec::new(),
        }
    }
}

impl SampledDataBars {
    pub fn set_day_bars(&mut self, day_bars: Vec<OHLCVBar>) {
        self.day_bars = day_bars;
    }
    pub fn set_month_bars(&mut self, month_bars: Vec<OHLCVBar>) {
        self.month_bars = month_bars;
    }
    pub fn set_year_bars(&mut self, year_bars: Vec<OHLCVBar>) {
        self.year_bars = year_bars;
    }

    pub fn day_bars(&self) -> &Vec<OHLCVBar> {
        &self.day_bars
    }
    pub fn month_bars(&self) -> &Vec<OHLCVBar> {
        &self.month_bars
    }
    pub fn year_bars(&self) -> &Vec<OHLCVBar> {
        &self.year_bars
    }
}

///
/// Trait for types that support indexing date components.
///
/// # Methods
///
/// * `get_current_year_index` - Get current year index
/// * `get_current_month_index` - Get current month index
/// * `get_current_day_index` - Get current day index
/// * `set_current_year_index` - Set current year index
/// * `set_current_month_index` - Set current month index
/// * `set_current_day_index` - Set current day index
///
/// * `get_previous_year_index` - Get previous year index
/// * `get_previous_month_index` - Get previous month index
/// * `get_previous_day_index` - Get previous day index
/// * `set_previous_year_index` - Set previous year index
/// * `set_previous_month_index` - Set previous month index
/// * `set_previous_day_index` - Set previous day index
///
/// The index values are `usize` representing array positions for year, month and day.
///
/// This allows storing date component values as global context indices.
///
pub trait Indexable {
    fn get_current_year_index(&self) -> usize;
    fn get_current_month_index(&self) -> usize;
    fn get_current_day_index(&self) -> usize;
    //
    fn set_current_year_index(&mut self, index: usize);
    fn set_current_month_index(&mut self, index: usize);
    fn set_current_day_index(&mut self, index: usize);

    fn get_previous_year_index(&self) -> usize;
    fn get_previous_month_index(&self) -> usize;
    fn get_previous_day_index(&self) -> usize;
    //
    fn set_previous_year_index(&mut self, index: usize);
    fn set_previous_month_index(&mut self, index: usize);
    fn set_previous_day_index(&mut self, index: usize);
}

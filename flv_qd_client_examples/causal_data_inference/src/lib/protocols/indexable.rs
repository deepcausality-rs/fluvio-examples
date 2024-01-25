use deep_causality::prelude::TimeScale;


pub trait Indexable {
    /// Get the index for the given key.
    ///
    /// # Parameters
    ///
    /// * `key` - The key representing the index type, typically an enum value like `TimeScale`.
    /// * `current` - Whether to get the current or previous index.
    ///
    /// # Returns
    ///
    /// The index value for the given key, typically a `usize` array position.
    ///
    fn get_index(&self, key: usize, current: bool) -> usize;

    /// Set the index for the given key.
    ///
    /// # Parameters
    ///
    /// * `key` - The key representing the index type, typically an enum value like `TimeScale`.
    /// * `index` - The index value to set, typically a `usize` array position.
    /// * `current` - Whether to set the current or previous index.
    ///
    fn set_index(&mut self, key: usize, index: usize, current: bool);
}


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
pub trait TimeIndexable: Indexable {
    /// Get the current year index.
    ///
    /// # Returns
    ///
    /// The current year index as a `usize`.
    ///
    fn get_current_year_index(&self) -> usize {
        self.get_index(TimeScale::Year as usize, true)
    }

    /// Get the current month index.
    ///
    /// # Returns
    ///
    /// The current month index as a `usize`.
    ///
    fn get_current_month_index(&self) -> usize {
        self.get_index(TimeScale::Month as usize, true)
    }

    /// Set the current year index.
    ///
    /// # Parameters
    ///
    /// * `index` - The year index to set as a `usize`
    ///
    fn set_current_year_index(&mut self, index: usize) {
        self.set_index(TimeScale::Year as usize, index, true)
    }

    /// Set the current month index.
    ///
    /// # Parameters
    ///
    /// * `index` - The month index to set as a `usize`
    ///
    fn set_current_month_index(&mut self, index: usize) {
        self.set_index(TimeScale::Month as usize, index, true)
    }

    /// Get the previous year index.
    ///
    /// # Returns
    ///
    /// The previous year index as a `usize`.
    ///
    fn get_previous_year_index(&self) -> usize {
        self.get_index(TimeScale::Year as usize, false)
    }

    /// Get the previous month index.
    ///
    /// # Returns
    ///
    /// The previous month index as a `usize`.
    ///
    fn get_previous_month_index(&self) -> usize {
        self.get_index(TimeScale::Month as usize, false)
    }

    /// Set the previous year index.
    ///
    /// # Parameters
    ///
    /// * `index` - The year index to set as a `usize`
    ///
    fn set_previous_year_index(&mut self, index: usize) {
        self.set_index(TimeScale::Year as usize, index, false)
    }

    /// Set the previous month index.
    ///
    /// # Parameters
    ///
    /// * `index` - The month index to set as a `usize`
    ///
    fn set_previous_month_index(&mut self, index: usize) {
        self.set_index(TimeScale::Month as usize, index, false)
    }
}

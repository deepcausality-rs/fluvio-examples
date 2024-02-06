use crate::types::alias::CustomContext;
use deep_causality::prelude::{Indexable, TimeScale};

/// Implements the [`TimeIndexExt`] trait for [`CustomContext`].
///
/// This allows a [`CustomContext`] to use the time indexing
/// functionality from [`TimeIndexExt`], like getting/setting
/// the current and previous year and month indices.
///
/// The default implementation of [`TimeIndexExt`] is located in:
/// lib/protocols/time_indexable
///
/// # Example
///
/// ```
/// use causal_model::prelude::{CustomContext, TimeIndexExt};
///
/// let mut context = CustomContext::with_capacity(1,"TestContext", 10);
///
/// context.set_current_year_index(2022);
/// let current_year = context.get_current_year_index();
/// ```
impl TimeIndexExt for CustomContext<'_> {}

///
/// Trait for types that support indexing date components.
///
/// # Methods
///
/// * `get_current_year_index` - Get current year index
/// * `get_current_month_index` - Get current month index
/// * `set_current_year_index` - Set current year index
/// * `set_current_month_index` - Set current month index
///
/// * `get_previous_year_index` - Get previous year index
/// * `get_previous_month_index` - Get previous month index
/// * `set_previous_year_index` - Set previous year index
/// * `set_previous_month_index` - Set previous month index
///
/// The index values are `usize` representing array positions for year, month and day.
///
/// This allows storing date component values as global context indices.
///
pub trait TimeIndexExt: Indexable {
    /// Get the current year index.
    ///
    /// # Returns
    ///
    /// The current year index as a `usize`.
    ///
    fn get_current_year_index(&self) -> Option<&usize> {
        let key = TimeScale::Year as usize;
        self.get_index(&key, true)
    }

    /// Get the current month index.
    ///
    /// # Returns
    ///
    /// The current month index as a `usize`.
    ///
    fn get_current_month_index(&self) -> Option<&usize> {
        let key = TimeScale::Month as usize;
        self.get_index(&key, true)
    }

    /// Set the current year index.
    ///
    /// # Parameters
    ///
    /// * `index` - The year index to set as a `usize`
    ///
    fn set_current_year_index(&mut self, index: usize) {
        let key = TimeScale::Year as usize;
        self.set_index(key, index, true)
    }

    /// Set the current month index.
    ///
    /// # Parameters
    ///
    /// * `index` - The month index to set as a `usize`
    ///
    fn set_current_month_index(&mut self, index: usize) {
        let key = TimeScale::Month as usize;
        self.set_index(key, index, true)
    }

    /// Get the previous year index.
    ///
    /// # Returns
    ///
    /// The previous year index as a `usize`.
    ///
    fn get_previous_year_index(&self) -> Option<&usize> {
        let key = TimeScale::Year as usize;
        self.get_index(&key, false)
    }

    /// Get the previous month index.
    ///
    /// # Returns
    ///
    /// The previous month index as a `usize`.
    ///
    fn get_previous_month_index(&self) -> Option<&usize> {
        let key = TimeScale::Month as usize;
        self.get_index(&key, false)
    }

    /// Set the previous year index.
    ///
    /// # Parameters
    ///
    /// * `index` - The year index to set as a `usize`
    ///
    fn set_previous_year_index(&mut self, index: usize) {
        let key = TimeScale::Year as usize;
        self.set_index(key, index, false)
    }

    /// Set the previous month index.
    ///
    /// # Parameters
    ///
    /// * `index` - The month index to set as a `usize`
    ///
    fn set_previous_month_index(&mut self, index: usize) {
        let key = TimeScale::Month as usize;
        self.set_index(key, index, false)
    }
}

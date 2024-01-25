use crate::prelude::TimeIndexable;
use crate::types::alias::CustomContext;

/// Implements the [`TimeIndexable`] trait for [`CustomContext`].
///
/// This allows a [`CustomContext`] to use the time indexing
/// functionality from [`TimeIndexable`], like getting/setting
/// the current and previous year and month indices.
///
/// The default implementation of [`TimeIndexable`] is located in:
/// lib/protocols/indexable.rs
///
/// # Example
///
/// ```
/// use lib_inference::prelude::{CustomContext, TimeIndexable};
///
/// let mut context = CustomContext::with_capacity(1,"TestContext", 10);
/// context.set_current_year_index(2022);
/// let current_year = context.get_current_year_index();
/// ```
impl TimeIndexable for CustomContext<'_> {}

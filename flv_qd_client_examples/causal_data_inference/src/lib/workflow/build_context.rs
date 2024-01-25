use crate::prelude::{CustomContext, SampledDataBars, TimeIndexable};
use crate::utils::counter;
use crate::utils::time_utils;
use crate::workflow::augment_data;
use chrono::Datelike;
use deep_causality::prelude::{
    Context, Contextoid, ContextoidType, ContextuableGraph, RelationKind, Root, TimeScale,
};
use std::error::Error;

/// Builds a time series context graph from OHLCV bar data.
///
/// Iterates through the provided OHLCV bar data, converting each bar to
/// augmented [`Time`] and [`RangeData`] nodes. Connects these nodes in a
/// hierarchy based on the provided [`TimeScale`].
///
/// The root node represents the entire time series. Year nodes are always added
/// under the root. Monthly nodes are optionally added under each Year if the
/// control map indicates Months should be included for the [`TimeScale`].
///
/// # Arguments
///
/// * `data` - The OHLCV bar data to build the context from.
/// * `time_scale` - The [`TimeScale`] to use for building the hierarchy.
/// * `node_capacity` - Max number of nodes to allocate for the context.
///
/// # Returns
///
/// The built [`Context`] graph representing the time series data.
///
pub fn build_time_data_context<'l>(
    data: &SampledDataBars,
    time_scale: &TimeScale,
    node_capacity: usize,
) -> Result<CustomContext<'l>, Box<dyn Error>> {
    // Create new atomic counter
    let counter = counter::RelaxedAtomicCounter::new();

    // Create new context
    let mut g = Context::with_capacity(1, "Causal Context", node_capacity);

    // Initialize indices to zero to prevent unnecessary unwrap errors
    g.set_current_month_index(0);
    g.set_current_year_index(0);

    // Create new time scale control map
    let cm = time_utils::get_time_scale_control_map(time_scale);
    let add_month = *cm.get(2).unwrap();

    // == ADD ROOT ==//
    let id = counter.increment_and_get();
    let root = Root::new(id);
    let root_node = Contextoid::new(id, ContextoidType::Root(root));
    let root_index = g.add_node(root_node);

    // == ADD YEAR ==//
    let time_scale = TimeScale::Year;
    let elements = data.year_bars();
    for data_bar in elements {
        let year = data_bar.date_time().year();

        //
        let (tempoid, dataoid) = augment_data::convert_ohlcv_bar_to_augmented(data_bar, time_scale);

        // Create new time node
        let key = counter.increment_and_get();
        let time_node = Contextoid::new(key, ContextoidType::Tempoid(tempoid));
        let year_index = g.add_node(time_node);

        // Set index of current year
        // Set index of previous year if current month is not the first year

        // Unwrap is safe because current year has been initialized to zero at the beginning of the function
        let current_year_index = *g.get_current_year_index().unwrap();

        if current_year_index != year_index {
            let prev_year_index = g.get_current_year_index().unwrap();

            g.set_previous_year_index(*prev_year_index);
            g.set_current_year_index(year_index);
        } else {
            // Set just index of current and previous year if current year is the first year
            g.set_current_year_index(year_index);
        }

        // Create new data node
        let data_id = counter.increment_and_get();
        let data_node = Contextoid::new(data_id, ContextoidType::Datoid(dataoid));
        let data_index = g.add_node(data_node);

        // link root to year
        g.add_edge(root_index, year_index, RelationKind::Temporal)
            .expect("Failed to add edge between root and year.");

        // link data to year
        g.add_edge(data_index, year_index, RelationKind::Datial)
            .expect("Failed to add edge between year and data");

        if !add_month {
            continue;
        }

        // == ADD MONTH FOR EACH YEAR ==//
        let time_scale = TimeScale::Month;
        let elements = data.month_bars();
        for data_bar in elements {
            if data_bar.date_time().year() != year {
                continue;
            }

            let (tempoid, dataoid) =
                augment_data::convert_ohlcv_bar_to_augmented(data_bar, time_scale);

            // Add Month
            let key = counter.increment_and_get();
            let time_node = Contextoid::new(key, ContextoidType::Tempoid(tempoid));
            let month_index = g.add_node(time_node);

            // Set index of current month
            // Set index of previous month if current month is not the first month

            // Unwrap is safe because current month has been initialized to zero at the beginning of the function
            let current_month_index = *g.get_current_month_index().unwrap();

            if current_month_index != month_index {
                let prev_month_index = g.get_current_month_index().unwrap();

                g.set_previous_month_index(*prev_month_index);
                g.set_current_month_index(month_index);
            } else {
                // Set index of current and previous month if current month is the first month
                g.set_current_month_index(month_index);
            }

            // Add data
            let data_id = counter.increment_and_get();
            let data_node = Contextoid::new(data_id, ContextoidType::Datoid(dataoid));
            let data_index = g.add_node(data_node);

            // link month to year
            g.add_edge(month_index, year_index, RelationKind::Temporal)
                .expect("Failed to add edge between month and year.");

            // link data to month
            g.add_edge(data_index, month_index, RelationKind::Datial)
                .expect("Failed to add edge between month and data.");
        } // end month
    } // end year

    Ok(g)
}

use crate::prelude::{CustomContext, TimeIndexable};
use crate::utils::{context_utils, counter, time_utils};
use chrono::Datelike;
use common::prelude::SampledDataBars;
use deep_causality::prelude::{
    Context, Contextoid, ContextoidType, ContextuableGraph, RelationKind, Root, TimeScale,
};
use std::error::Error;

/// Builds a time series context graph from OHLCV bar data.
///
/// Takes in OHLCV bar data, a time scale, and node capacity.
/// Iterates through the bar data and converts each one to augmented time and data nodes.
/// Connects the nodes in a hierarchy based on the provided time scale.
///
/// The root node represents the entire time series.
/// Year nodes are added under root.
/// Monthly nodes optionally added under Years based on time scale control map.
///
/// Arguments:
///
/// - data: The OHLCV bar data to build context from.
/// - time_scale: The TimeScale to use for hierarchy.
/// - node_capacity: Max nodes to allocate.
///
/// Returns:
///
/// - The built Context graph representing the time series data.
///
/// Logic:
///
/// 1. Create new context graph and init year/month indices.
/// 2. Add root node for entire series.
/// 3. Group bars by year, convert to nodes, add year nodes under root.
/// 4. Set current/previous year indices.
/// 5. If time scale includes months, group bars by month, add month nodes under years.
/// 6. Set current/previous month indices.
/// 7. Connect nodes in hierarchy based on time scale.
/// 8. Return built context graph.
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
        // Get current year from bar
        let year = data_bar.date_time().year();

        // Augment OHLCV bar with time and data nodes
        let (tempoid, dataoid) =
            context_utils::convert_ohlcv_bar_to_augmented(data_bar, time_scale);

        // Create year time node
        let key = counter.increment_and_get();
        let time_node = Contextoid::new(key, ContextoidType::Tempoid(tempoid));
        let year_time_index = g.add_node(time_node);

        // Create year data node
        let data_id = counter.increment_and_get();
        let data_node = Contextoid::new(data_id, ContextoidType::Datoid(dataoid));
        let year_data_index = g.add_node(data_node);

        // Set index of data for current year
        // Set index of previous year if current year is not the first year
        //
        let current_year_index = *g.get_current_year_index().unwrap_or_else(|| &0);

        if current_year_index != year_data_index {
            let prev_year_index = *g.get_current_year_index().unwrap_or_else(|| &0);

            if current_year_index != prev_year_index {
                g.set_previous_year_index(prev_year_index);

            }

            g.set_current_year_index(year_data_index);
        } else {
            // Set just index of current and previous year if current year is the first year
            g.set_current_year_index(year_data_index);
        }

        // link root to year
        g.add_edge(root_index, year_time_index, RelationKind::Temporal)
            .expect("Failed to add edge between root and year.");

        // link data to year
        g.add_edge(year_data_index, year_time_index, RelationKind::Datial)
            .expect("Failed to add edge between year and data");

        if !add_month {
            continue;
        }

        // == ADD MONTH FOR EACH YEAR ==//
        let time_scale = TimeScale::Month;
        let elements = data.month_bars();
        for data_bar in elements {
            //
            // Skip bars that are not in current year
            if data_bar.date_time().year() != year {
                continue;
            }

            // Augment OHLCV bar with time and data nodes
            let (tempoid, dataoid) =
                context_utils::convert_ohlcv_bar_to_augmented(data_bar, time_scale);

            // Add data
            let data_id = counter.increment_and_get();
            let data_node = Contextoid::new(data_id, ContextoidType::Datoid(dataoid));
            let month_data_index = g.add_node(data_node);

            // Add Month
            let key = counter.increment_and_get();
            let time_node = Contextoid::new(key, ContextoidType::Tempoid(tempoid));
            let month_time_index = g.add_node(time_node);

            // Set index of data from current month
            // Set index of previous month if current month is not the first month
            //
            let current_month_index = *g.get_current_month_index().unwrap_or_else(|| &0);

            if current_month_index != month_data_index {
                let prev_month_index = *g
                    .get_current_month_index()
                    .unwrap_or_else(|| &0);

                if current_month_index != prev_month_index {
                    g.set_previous_month_index(prev_month_index);
                }

                // g.set_previous_month_index(*prev_month_index);
                g.set_current_month_index(month_data_index);
            } else {
                // Set index of current and previous month if current month is the first month
                g.set_current_month_index(month_data_index);
            }

            // link month to year
            g.add_edge(month_time_index, year_time_index, RelationKind::Temporal)
                .expect("Failed to add edge between month and year.");

            // link data to month
            g.add_edge(month_data_index, month_time_index, RelationKind::Datial)
                .expect("Failed to add edge between month and data.");
        } // end month
    } // end year

    Ok(g)
}

use crate::prelude::{CustomContext, TimeIndexExt};
use crate::utils::{context_utils, time_utils};
use chrono::Datelike;
use client_utils::prelude::atomic_counter;
use common::prelude::SampledDataBars;
use deep_causality::prelude::{
    Context, Contextoid, ContextoidType, ContextuableGraph, RelationKind, Root, TimeScale,
};
use std::error::Error;

const FN_NAME: &str = "[build_time_data_context]: ";

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
    //
    // println!("{FN_NAME}: Creating a new atomic counter.");
    let counter = atomic_counter::RelaxedAtomicCounter::new();

    // println!("{FN_NAME}: Creating a new context.");
    let mut g = Context::with_capacity(1, "Causal Context", node_capacity);

    // println!("{FN_NAME}: Creating a new time scale control map.");
    let cm = time_utils::get_time_scale_control_map(time_scale);
    let add_month = *cm.get(2).unwrap();

    // == INIT YEAR/MONTH INDICES ==//
    // println!("{FN_NAME}: Initializing year and month indices to zero.");
    g.set_current_year_index(0);
    g.set_current_month_index(0);

    // == ADD ROOT ==//
    // println!("{FN_NAME}: Adding root node.");
    let id = counter.increment_and_get();
    let root = Root::new(id);
    let root_node = Contextoid::new(id, ContextoidType::Root(root));
    let root_index = g.add_node(root_node);
    // println!("{FN_NAME}: root node index {}", root_index);

    // == ADD YEAR ==//
    let time_scale = TimeScale::Year;
    let elements = data.year_bars();
    for data_bar in elements {
        // Get year from bar
        let year = data_bar.date_time().year();
        // println!("{FN_NAME}: year:{}", &year);

        // Augment OHLCV bar with time and data nodes
        // println!("{FN_NAME}: Augmenting data bar to augmented nodes.");
        let (tempoid, dataoid) =
            context_utils::convert_ohlcv_bar_to_augmented(data_bar, time_scale);

        // Create year time node
        // println!("{FN_NAME}: Creating year time node.");
        let key = counter.increment_and_get();
        let time_node = Contextoid::new(key, ContextoidType::Tempoid(tempoid));
        let year_time_index = g.add_node(time_node);
        // println!("{FN_NAME}: year time node index {}", year_time_index);

        // Create year data node
        // println!("{FN_NAME}: Creating year data node.");
        let data_id = counter.increment_and_get();
        let data_node = Contextoid::new(data_id, ContextoidType::Datoid(dataoid));
        let year_data_index = g.add_node(data_node);
        // println!("{FN_NAME}: year data node index {}", year_data_index);

        // Set index of previous year if current year is not the the same as before
        let current_year_index = *g
            .get_current_year_index()
            .unwrap_or_else(|| panic!("{FN_NAME} Failed to get current year index."));

        // println!("{FN_NAME}: current year index stored: {}.", current_year_index);

        if current_year_index != year_data_index {
            // println!("{FN_NAME}: current year is not the the same as before.");

            // println!("{FN_NAME}: Setting current year index to: {} ", year_data_index);
            g.set_current_year_index(year_data_index);

            let prev_year_index = *g
                .get_current_year_index()
                .expect("[build_time_data_context]: Failed to get previous year index.");

            // println!("{FN_NAME}: previous year index stored: {}.", prev_year_index);

            if current_year_index != prev_year_index {
                // println!("{FN_NAME}: previous year is not the the same as in store.");
                // println!("{FN_NAME}: Setting previous year index to: {} ", prev_year_index);
                g.set_previous_year_index(prev_year_index);
            }
        }

        // println!("{FN_NAME}: Linking root to year.");
        g.add_edge(root_index, year_time_index, RelationKind::Temporal)
            .expect("Failed to add edge between root and year.");

        // println!("{FN_NAME}: Linking year to data.");
        g.add_edge(year_data_index, year_time_index, RelationKind::Datial)
            .expect("Failed to add edge between year and data");

        if !add_month {
            continue;
        }

        // == ADD MONTH FOR EACH YEAR ==//
        // println!("{FN_NAME}: Adding month nodes for each year.");
        let time_scale = TimeScale::Month;
        let elements = data.month_bars();
        for data_bar in elements {
            //
            // Skip bars that are not in current year
            if data_bar.date_time().year() != year {
                continue;
            }

            // println!("{FN_NAME}: Augmenting data bar to augmented nodes.");
            let (tempoid, dataoid) =
                context_utils::convert_ohlcv_bar_to_augmented(data_bar, time_scale);

            // Add data
            // println!("{FN_NAME}: Creating month time node.");
            let data_id = counter.increment_and_get();
            let data_node = Contextoid::new(data_id, ContextoidType::Datoid(dataoid));
            let month_data_index = g.add_node(data_node);
            // println!("{FN_NAME}: month data node index {}", month_data_index);

            // Add Month
            let key = counter.increment_and_get();
            let time_node = Contextoid::new(key, ContextoidType::Tempoid(tempoid));
            let month_time_index = g.add_node(time_node);
            // println!("{FN_NAME}: month time node index {}", month_time_index);

            let current_month_index = *g.get_current_month_index().unwrap_or(&0);
            // println!("{FN_NAME}: current month index stored: {}.", current_month_index);

            if current_month_index != month_data_index {
                // println!("{FN_NAME}: current month is not the the same as before.");

                // println!("{FN_NAME}: Setting current month index to: {} ", month_data_index);
                g.set_current_month_index(month_data_index);

                let prev_month_index = *g.get_current_month_index().unwrap_or(&0);
                // println!("{FN_NAME}: previous month index stored: {}.", prev_month_index);

                if current_month_index != prev_month_index {
                    // println!("{FN_NAME}: previous month is not the the same as in store.");
                    // println!("{FN_NAME}: Setting previous month index to: {} ", prev_month_index);
                    g.set_previous_month_index(prev_month_index);
                }
            }

            // println!("{FN_NAME}: Linking month to year.");
            g.add_edge(month_time_index, year_time_index, RelationKind::Temporal)
                .expect("Failed to add edge between month and year.");

            // println!("{FN_NAME}: Linking data to month.");
            g.add_edge(month_data_index, month_time_index, RelationKind::Datial)
                .expect("Failed to add edge between month and data.");
        } // end month
    } // end year

    Ok(g)
}

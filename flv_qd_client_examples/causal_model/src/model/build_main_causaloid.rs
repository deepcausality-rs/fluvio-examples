use crate::model::causaloid_range_month::get_month_causaloid;
use crate::model::causaloid_range_year::get_year_causaloid;
use crate::prelude::{CustomCausaloid, CustomContext};
use deep_causality::prelude::{CausableGraph, Causaloid, CausaloidGraph};

/// Creates the main causaloid graph from the provided context.
///
/// # Arguments
///
/// * `context` - The context graph to build the causaloid from.
///
/// # Returns
///
/// The built causaloid graph containing:
///
/// - A root causaloid node representing the entire time series.
/// - A month causaloid node connected under the root.
///
///
/// The causaloid graph connects high-level time aggregations
/// (years, months) to enable causal reasoning on different time scales.
///
/// Additional causaloid nodes can be added to the graph as needed.
///
pub fn build_main_causaloid<'l>(context: &'l CustomContext<'l>) -> CustomCausaloid<'l> {
    let mut g = CausaloidGraph::new();

    // Add the root causaloid to the causaloid graph
    let root_causaloid = get_year_causaloid(context, 1);
    let root_index = g.add_root_causaloid(root_causaloid);

    // Add the month causaloid to the causaloid graph
    let month_causaloid = get_month_causaloid(context, 2);
    let month_index = g.add_causaloid(month_causaloid);

    let _ = g.add_edge(root_index, month_index);

    // Here we wrap the causal graph into a causaloid
    Causaloid::from_causal_graph_with_context(0, g, Option::from(context), "Causaloid main graph")
}

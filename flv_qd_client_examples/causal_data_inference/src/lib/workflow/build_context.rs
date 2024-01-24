use std::error::Error;
use deep_causality::prelude::{Context, Contextoid, ContextoidType, ContextuableGraph, Root};
use common::prelude::TimeScale;
use crate::prelude::{CustomContext, SampledDataBars};
use crate::utils::counter;
use crate::utils::time_scale_utils;

pub fn build_time_data_context<'l>(
    data: &SampledDataBars,
    max_time_scale: TimeScale,
    node_capacity: usize,
) -> Result<CustomContext<'l>, Box<dyn Error>> {
    let context = match build_time_data_context_graph(data, max_time_scale, node_capacity) {
        Ok(g) => g,
        Err(e) => return Err(e),
    };

    Ok(context)
}

fn build_time_data_context_graph<'l>(
    data: &SampledDataBars,
    time_scale: TimeScale,
    node_capacity: usize,
) -> Result<CustomContext<'l>, Box<dyn Error>> {

    // Create new atomic counter
    let counter = counter::RelaxedAtomicCounter::new();

    // Create new context
    let mut g = Context::with_capacity(1, "Causal Context", node_capacity);

    // Create new time scale control map
    let cm = time_scale_utils::get_boolean_control_map(time_scale);
    let add_month = *cm.get(2).unwrap();
    let add_day = *cm.get(4).unwrap();

    // == ADD ROOT ==//
    let id = counter.increment_and_get();
    let root = Root::new(id);
    let root_node = Contextoid::new(id, ContextoidType::Root(root));
    let _root_index = g.add_node(root_node);

    // == ADD YEAR ==//
    //let time_scale = TimeScale::Year;

    Ok(g)
}
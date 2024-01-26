use crate::prelude::{BarRange, CustomContext, Rangeable};
use deep_causality::errors::CausalityError;
use deep_causality::prelude::{Contextuable, ContextuableGraph};

/// Extracts data from a context node.
///
/// # Arguments
///
/// * `ctx` - A reference to the context containing the node to extract data from
/// * `index` - The index of the node in the context
///
/// # Returns
///
/// The BarRange data contained in the node's dataoid field.
///
/// # Errors
///
/// Returns a CausalityError if the node or its dataoid is not found.
///
pub fn extract_data_from_context<'l>(
    ctx: &'l CustomContext<'l>,
    index: usize,
) -> Result<BarRange, CausalityError> {
    //Get node from context.
    let node = ctx
        .get_node(index)
        .expect(format!("Node for the index {} not found in context", index).as_str());

    // Extract the data from the node.
    let data = node
        .vertex_type()
        .dataoid()
        .expect(format!("No Data for node at index {}", index).as_str());

    // Deref and return.
    Ok(data.data_range())
}
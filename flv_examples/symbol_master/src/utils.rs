use common::prelude::HostEndpoint;

/// Returns the configuration for the SymdbClient.
///
/// This creates a new HostEndpoint with:
/// - host: "0.0.0.0"
/// - port: 7070
///
/// # Returns
///
/// A HostEndpoint struct containing the SymdbClient configuration.
///
/// # Example
///
/// ```
/// let config = get_symdb_config();
/// let client = SymdbClient::new(config);
/// ```
///
pub(crate) fn get_symdb_config() -> HostEndpoint {
    HostEndpoint::new("0.0.0.0".to_string(), 7070)
}

use common::prelude::ServiceID;

pub fn print_start_header_grpc_service(
    service_id: &ServiceID,
    service_addr: &str,
    metrics_addr: &str,
    metrics_uri: &str,
) {
    println!("||  {}  ||", service_id);
    println!("==========================================");
    println!("Service on endpoint: {}", service_addr);
    println!("Metrics on endpoint: {}/{}", metrics_addr, metrics_uri);
    println!("==========================================");
    println!();
}

/// Prints a start header for a message-based service.
///
/// # Arguments
///
/// * `service_id` - The ServiceID of the service
/// * `service_topic` - The topic the service is listening on
/// * `metrics_addr` - The address for the metrics endpoint
/// * `metrics_uri` - The URI for the metrics endpoint
///
/// # Remarks
///
/// Prints a formatted header with:
/// - ServiceID
/// - Listening topic
/// - Metrics address and URI
///
/// Indicates message-based service has started up.
///
pub fn print_start_header_message_service(
    service_id: &ServiceID,
    service_topic: &str,
    metrics_addr: &str,
    metrics_uri: &str,
) {
    println!("||  {}  ||", service_id);
    println!("==========================================");
    println!("Listening on topic: {}", service_topic);
    println!("Metrics on endpoint: {}/{}", metrics_addr, metrics_uri);
    println!("==========================================");
    println!();
}

/// Prints a stop header when a service shuts down.
///
/// # Arguments
///
/// * `service_id` - The ServiceID of the stopping service
///
/// # Remarks
///
/// Prints a formatted header with:
/// - ServiceID
/// - Shutdown complete message
///
/// Indicates clean shutdown of the service.
///
pub fn print_stop_header(service_id: &ServiceID) {
    println!();
    println!("==========================================");
    println!("{} service shutdown complete", service_id);
    println!("==========================================");
}

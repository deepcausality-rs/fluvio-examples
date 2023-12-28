use common::prelude::ServiceID;

pub fn print_start_header(
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

pub fn print_stop_header(service_id: &ServiceID) {
    println!();
    println!("==========================================");
    println!("{} service shutdown complete", service_id);
    println!("==========================================");
}

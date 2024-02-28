pub mod click_house_config;
/// Configuration type definitions
///
/// This module provides configuration data structures used throughout the
/// application. It contains the following modules:
///
/// - `client_channel`: Configuration for client channels
/// - `db_config`: Database configuration
/// - `environment_types`: Configuration for environments like development or production
/// - `host_endpoint`: Configuration for host endpoints
/// - `message_client_config`: Configuration for message clients
/// - `metric_config`: Configuration for metrics and monitoring
/// - `service_config`: Configuration for services
/// - `service_id`: Configuration for service IDs
///
///
/// The modules contain type definitions rather than functional logic. The
/// purpose is to centralize and document the application's configuration
/// data structures.
///
pub mod client_channel;
pub mod db_config;
pub mod environment_types;
pub mod host_endpoint;
pub mod iggy_user;
pub mod message_client_config;
pub mod metric_config;
pub mod service_config;
pub mod service_id;

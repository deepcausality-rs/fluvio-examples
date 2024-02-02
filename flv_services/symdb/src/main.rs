mod service;

use crate::service::SYMDBServer;
use autometrics::prometheus_exporter;
use common::prelude::ServiceID;
use config_manager::ConfigManager;
use db_query_manager::QueryDBManager;
use proto::binding::symdb_service_server::SymdbServiceServer;
use service_utils::{print_utils, shutdown_utils};
use std::net::SocketAddr;
use std::sync::{Arc, RwLock};
use symbol_manager::SymbolManager;
use tonic::transport::Server;
use warp::Filter;

const SVC_ID: ServiceID = ServiceID::SYMDB;

/// This module sets up and starts the SYMDB gRPC service and metrics server.
///
/// ## Initialization
///
/// - Initializes the Prometheus metrics exporter.
/// - Creates a `ConfigManager` instance to handle configuration.
///
/// ## Metrics Server Setup
///
/// - Gets metrics host, port, and URI from `ConfigManager`.
/// - Creates a `SocketAddr` for the metrics server.
/// - Creates a Warp filter to handle Prometheus metric requests.
///
/// ## gRPC Service Setup
///
/// - Gets the symbol table for the default exchange from `ConfigManager`.
/// - Creates a `QueryDBManager` to fetch symbols from the database.
/// - Fetches all symbols for the default exchange.
/// - Creates a `SymbolManager` to manage the symbol data.
/// - Closes `QueryDBManager` as it is no longer needed.
/// - Configures the gRPC service address from `ConfigManager`.
/// - Creates the gRPC service with a `SYMDBServer`.
/// - Adds a health service to the gRPC server.
///
/// ## Starting Servers
///
/// - Starts the gRPC server and metrics server concurrently.
/// - Spawns each as a `tokio` task.
/// - Collects the results to check for errors.
///
/// ## On shutdown:
///
/// - Shuts down HTTP server
/// - Shuts down gRPC server
/// - Prints service stop message
///
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the metrics exporter.
    prometheus_exporter::init();

    // Setup ConfigManager instance for contextual autoconfiguration.
    let cfg_manager = async { ConfigManager::new(SVC_ID) }.await;

    // Configure the metrics endpoint.
    let metric_config = cfg_manager.svc_metric_config();
    let metrics_host = metric_config.metric_host();
    let metrics_port = metric_config.metric_port();
    let metrics_uri = metric_config.metric_uri();
    let metrics_addr = format!("{}:{}", metrics_host, metrics_port);

    //Creates a SocketAddr instance from the metrics address string.
    let web_addr: SocketAddr = metrics_addr
        .parse()
        .expect("[SYMDB]/main: Failed to parse metric host to address");

    //Creates a new Warp filter for the metrics endpoint.
    let routes = warp::get()
        .and(warp::path(metrics_uri.clone()))
        .map(prometheus_exporter::encode_http_response);

    //Creates a new Warp filter for the metrics endpoint with a graceful shutdown handlers.
    let signal = shutdown_utils::signal_handler("Http web server");
    let (_, web_server) = warp::serve(routes).bind_with_graceful_shutdown(web_addr, signal);

    // Get the symbol table for the default exchange.
    let default_exchange = cfg_manager.default_exchange();
    let exchanges = cfg_manager.exchanges_id_names().to_owned();
    let exchange_symbol_table = cfg_manager
        .get_symbol_table(default_exchange)
        .expect("[SYMDB]/main: Failed to get symbol table for default exchange.");

    // Create a new QueryDBManager instance.
    let db_config = cfg_manager.db_config();
    let mut q_manager = QueryDBManager::new(db_config)
        .await
        .expect("[SYMDB]/main: Failed to create QueryDBManager instance.");

    // Get all symbols for the default exchange.
    let symbols = q_manager
        .get_all_symbols_with_ids(&exchange_symbol_table)
        .await
        .expect("[SYMDB]/main: Failed to get all symbols for SymbolManager.");

    // Create a new SymbolManager instance.
    let symbol_manager = async {
        Arc::new(RwLock::new(SymbolManager::new(symbols, exchanges).expect(
            "[SYMDB]/main: Failed to create SymbolManager instance.",
        )))
    }
    .await;

    // Close the DB Connection as its not needed anymore.
    q_manager.close().await;

    // Configure & Construct gRPC via auto config
    let service_addr = cfg_manager.get_svc_socket_addr();

    // Set up socket address for gRPC service
    let grpc_addr = service_addr
        .parse()
        .expect("[CMDB]: Failed to parse address");

    // Create new gRPC service
    let grpc_svc = SymdbServiceServer::new(SYMDBServer::new(symbol_manager));

    // Build health service for gRPC server
    let (mut health_reporter, health_svc) = tonic_health::server::health_reporter();
    health_reporter
        .set_serving::<SymdbServiceServer<SYMDBServer>>()
        .await;

    // Build gRPC server with health service and signal sigint handler
    let signal = shutdown_utils::signal_handler("gRPC server");
    let grpc_server = Server::builder()
        .add_service(grpc_svc)
        .add_service(health_svc)
        .serve_with_shutdown(grpc_addr, signal);

    // print the start message on the console.
    print_utils::print_start_header_grpc_service(
        &SVC_ID,
        &service_addr,
        &metrics_addr,
        &metrics_uri,
    );

    // Free up some memory before starting the service,
    drop(cfg_manager);
    drop(metrics_host);
    drop(metrics_uri);
    drop(metrics_addr);
    drop(q_manager);
    drop(service_addr);

    //Creates a new Tokio task for each server.
    // https://github.com/hyperium/tonic/discussions/740
    let grpc_handle = tokio::spawn(grpc_server);
    let web_handle = tokio::spawn(web_server);

    //Starts both servers concurrently.
    match tokio::try_join!(grpc_handle, web_handle) {
        Ok(_) => {}
        Err(e) => {
            println!(
                "[SYMDB]/main: Failed to start Fluvio and HTTP/Metric server: {:?}",
                e
            );
        }
    }

    //Prints the stop headers for the current service.
    print_utils::print_stop_header(&SVC_ID);

    Ok(())
}

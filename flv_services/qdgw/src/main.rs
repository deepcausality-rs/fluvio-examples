use std::net::SocketAddr;
use std::sync::Arc;

use autometrics::prometheus_exporter;
use tokio::sync::RwLock;
use warp::Filter;

use common::prelude::ServiceID;
use config_manager::ConfigManager;
use db_query_manager::QueryDBManager;
use service_utils::{print_utils, shutdown_utils};
use symbol_manager::SymbolManager;

use crate::service::Server;

mod service;

const SVC_ID: ServiceID = ServiceID::QDGW;

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
        .expect("[QDGW]/main: Failed to parse metric host to address");

    //Creates a new Warp filter for the metrics endpoint.
    let routes = warp::get()
        .and(warp::path(metrics_uri.clone()))
        .map(prometheus_exporter::encode_http_response);

    //Creates a new Warp filter for the metrics endpoint with a graceful shutdown handlers.
    let signal = shutdown_utils::signal_handler("Http web server");
    let (_, web_server) = warp::serve(routes).bind_with_graceful_shutdown(web_addr, signal);

    //Creates a new Tokio task for the HTTP web server.
    let web_handle = tokio::spawn(web_server);

    // Get the symbol table for the default exchange.
    let default_exchange = cfg_manager.default_exchange();
    let exchanges = cfg_manager.exchanges_id_names().to_owned();
    let exchange_symbol_table = cfg_manager
        .get_symbol_table(default_exchange)
        .expect("[QDGW]/main: Failed to get symbol table for default exchange.");

    // Create a new QueryDBManager instance.
    let db_config = cfg_manager.db_config();
    let mut q_manager = QueryDBManager::new(db_config.clone())
        .await
        .expect("[QDGW]/main: Failed to create QueryDBManager instance.");

    // Get all symbols for the default exchange.
    let symbols = q_manager
        .get_all_symbols_with_ids(&exchange_symbol_table)
        .await
        .expect("[QDGW]/main: Failed to get all symbols for SymbolManager.");

    // Create a new SymbolManager instance.
    let symbol_manager = async {
        Arc::new(RwLock::new(
            SymbolManager::new(symbols, exchanges)
                .expect("[QDGW]/main: Failed to create SymbolManager instance."),
        ))
    }
    .await;

    // Wrap the QueryDBManager instance into an Arc/Mutex to allow multi-threaded access.
    let query_manager = Arc::new(RwLock::new(q_manager));

    // Check if the database connection is open.
    let q_manager = query_manager.read().await;
    let is_open = q_manager.is_open().await;

    if is_open {
        println!("✅ Database Connection OK!");
    }

    // Autoconfigure message bus connector.
    let iggy_config = cfg_manager.iggy_config();
    let service_topic = iggy_config.topic_name().to_string();

    //Creates a new server
    let server = Server::new(iggy_config, query_manager.clone(), symbol_manager).await;

    //Creates a new Tokio task for the server.
    let signal = shutdown_utils::signal_handler("Message Bus connector");
    let service_handle = tokio::spawn(server.run(signal));

    // Prints the start header for the service
    print_utils::print_start_header_message_service(
        &SVC_ID,
        &service_topic,
        &metrics_addr,
        &metrics_uri,
    );

    // Free up some memory before starting the service,
    drop(db_config);
    drop(cfg_manager);
    drop(metrics_host);
    drop(metrics_uri);
    drop(metrics_addr);

    //Starts both servers concurrently.
    match tokio::try_join!(web_handle, service_handle) {
        Ok(_) => {}
        Err(e) => {
            println!(
                "[QDGW]/main: Failed to start Message service and HTTP/Metric server: {:?}",
                e
            );
        }
    }

    //Prints the stop headers for the current service.
    print_utils::print_stop_header(&SVC_ID);

    Ok(())
}

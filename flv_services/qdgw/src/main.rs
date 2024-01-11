mod handle;
mod handle_clients;
mod handle_data_start;
mod handle_data_stop;
mod handle_data_stop_all;
mod handle_unknown_msg;
mod service;
mod utils;

use futures::lock::Mutex;
use std::net::SocketAddr;
use std::sync::Arc;
use warp::Filter;

use crate::service::Server;
use autometrics::prometheus_exporter;
use client_manager::ClientManager;
use common::prelude::ServiceID;
use config_manager::ConfigManager;
use db_query_manager::QueryDBManager;
use service_utils::{print_utils, shutdown_utils};
use symbol_manager::SymbolManager;

const SVC_ID: ServiceID = ServiceID::QDGW;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the metrics exporter.
    prometheus_exporter::init();

    // Setup ConfigManager instance for contextual autoconfiguration.
    let cfg_manager = async { ConfigManager::new(SVC_ID) }.await;

    // Configure the metrics endpoint.
    let metric_config = cfg_manager.get_svc_metric_config();
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

    //Creates a new Warp filter for the metrics endpoint with a graceful shutdown handler.
    let signal = shutdown_utils::signal_handler("Http web server");
    let (_, web_server) = warp::serve(routes).bind_with_graceful_shutdown(web_addr, signal);

    //Creates a new Tokio task for the HTTP web server.
    let web_handle = tokio::spawn(web_server);

    // Autoconfigures message channel
    let msg_config = cfg_manager.get_message_client_config();
    let service_topic = msg_config.control_channel();

    // Creates a new consumer for the topic
    let consumer = fluvio::consumer(&service_topic, 0)
        .await
        .expect("[QDGW]/main: Failed to create a message consumer");

    // We have to use Arc/Mutex here to allow multi-threaded access those manager instances.
    let client_manager = Arc::new(Mutex::new(ClientManager::new()));

    let db_config = cfg_manager.get_db_config();
    let mut q_manager = QueryDBManager::new(db_config)
        .await
        .expect("[QDGW]/main: Failed to create QueryDBManager instance.");

    // Move this to autoconfig.
    let exchanges = vec![(1, "kraken".to_string()), (2, "bittrex".to_string())];
    let symbol_table = "kraken_symbols";

    let symbols = q_manager
        .get_all_symbols_with_ids(symbol_table)
        .await
        .expect("[QDGW]/main: Failed to get all symbols for SymbolManager.");

    let symbol_manager = async {
        Arc::new(Mutex::new(
            SymbolManager::new(symbols, exchanges)
                .expect("[QDGW]/main: Failed to create SymbolManager instance."),
        ))
    }
    .await;

    let query_manager = Arc::new(Mutex::new(q_manager));

    //Creates a new server
    let server = Server::new(consumer, client_manager, query_manager, symbol_manager);

    //Creates a new Tokio task for the server.
    let signal = shutdown_utils::signal_handler("Fluvio connector");
    let service_handle = tokio::spawn(server.run(signal));

    // Prints the start header for the service
    print_utils::print_start_header_message_service(
        &SVC_ID,
        &service_topic,
        &metrics_addr,
        &metrics_uri,
    );

    //Starts both servers concurrently.
    match tokio::try_join!(web_handle, service_handle) {
        Ok(_) => {}
        Err(e) => {
            println!(
                "[QDGW]/main: Failed to start Fluvio and HTTP server: {:?}",
                e
            );
        }
    }

    //Prints the stop headers for the current service.
    print_utils::print_stop_header(&SVC_ID);

    Ok(())
}

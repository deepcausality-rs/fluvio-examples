mod handle_clients;
mod service;
mod handle_data;

use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use warp::Filter;

use autometrics::prometheus_exporter;
use client_manager::ClientManager;
use common::prelude::{MessageClientConfig, ServiceID};
use service_utils::{print_utils, shutdown_utils};
use crate::service::Server;

const SVC_ID: ServiceID = ServiceID::QDGW;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    prometheus_exporter::init();

    let metrics_host = "0.0.0.0";
    let metrics_port = 8080;
    let metrics_uri = "metrics";
    let metrics_addr = format!("{}:{}", metrics_host, metrics_port);

    //Creates a SocketAddr instance from the metrics address string.
    let web_addr: SocketAddr = metrics_addr
        .parse()
        .expect("[QDGW]/main: Failed to parse metric host to address");


    //Creates a new Warp filter for the metrics endpoint.
    let routes = warp::get()
        .and(warp::path(metrics_uri))
        .map(prometheus_exporter::encode_http_response);

    //Creates a new Warp filter for the metrics endpoint with a graceful shutdown handler.
    let signal = shutdown_utils::signal_handler("http web server");
    let (_, web_server) = warp::serve(routes).bind_with_graceful_shutdown(web_addr, signal);

    //Creates a new Tokio task for the HTTP web server.
    let web_handle = tokio::spawn(web_server);

    // Autoconfigures message channel
    let msg_config = MessageClientConfig::from_svc_id(SVC_ID);
    let service_topic = msg_config.control_channel();

    // creates a new consumer for the topic
    let consumer = fluvio::consumer(&service_topic, 0)
        .await
        .expect("[QDGW]/main: Failed to create a message consumer");

    let client_manager = Arc::new(Mutex::new(ClientManager::new()));

    //Creates a new server
    let server = Server::new(consumer, client_manager);

    //Creates a new Tokio task for the server.
    let signal = shutdown_utils::signal_handler("ZMQ server");
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
            println!("[QDGW]/main: Failed to start gRPC and HTTP server: {:?}", e);
        }
    }

    //Prints the stop headers for the current service.
    print_utils::print_stop_header(&SVC_ID);

    Ok(())
}

use autometrics::prometheus_exporter;
use common::prelude::ServiceID;
use config_manager::ConfigManager;
use db_query_manager::QueryDBManager;
use service_utils::{print_utils, shutdown_utils};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use warp::Filter;

const SVC_ID: ServiceID = ServiceID::SYMDB;

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

    //Creates a new Tokio task for the HTTP web server.
    let web_handle = tokio::spawn(web_server);

    // Create a new QueryDBManager instance.
    let db_config = cfg_manager.db_config();
    let q_manager = QueryDBManager::new(db_config.clone())
        .await
        .expect("[SYMDB]/main: Failed to create QueryDBManager instance.");

    // Wrap the QueryDBManager instance into an Arc/Mutex to allow multi-threaded access.
    let query_manager = Arc::new(Mutex::new(q_manager));

    // Check if the database connection is open.
    let q_manager = query_manager
        .lock()
        .expect("[SYMDB]/main: Failed to lock QueryDBManager");

    let is_open = q_manager.is_open().await;
    drop(q_manager);

    if is_open {
        println!("✅ Database Connection OK!");
    }

    print_utils::print_start_header_grpc_service(&SVC_ID, "", &metrics_addr, &metrics_uri);

    //Starts both servers concurrently.
    match tokio::try_join!(web_handle) {
        Ok(_) => {}
        Err(e) => {
            println!(
                "[SYMDB]/main: Failed to start Fluvio and HTTP/Metric server: {:?}",
                e
            );
        }
    }

    // Close the DB Connection pool.
    let q_manager = query_manager
        .lock()
        .expect("[SYMDB]/main: Failed to lock QueryDBManager");

    q_manager.close().await;
    println!("* Database connection closed");

    //Prints the stop headers for the current service.
    print_utils::print_stop_header(&SVC_ID);

    Ok(())
}

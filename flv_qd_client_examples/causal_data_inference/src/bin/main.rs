use client_utils::{handle_error_utils, handle_utils, print_utils};
use common::prelude::{MessageClientConfig, ServiceID};
use config_manager::ConfigManager;
use db_query_manager::QueryDBManager;
use deep_causality::prelude::{ContextuableGraph, Identifiable, TimeScale};
use lib_inference::prelude::{build_context, load_data, model};
use qd_client::QDClient;
use std::time::Duration;
use symbol_manager::SymbolManager;
use tokio::time::sleep;

const EXAMPLE: &'static str = "Causal Data Inference";

const FN_NAME: &'static str = "causal_data_inference/main";

const CLIENT_ID: u16 = 77;

#[tokio::main]
async fn main() {
    print_utils::print_example_header(EXAMPLE);

    println!("{FN_NAME}: Creating a new ConfigManager.");
    let cfg_manager = async { ConfigManager::new(ServiceID::Default) }.await;
    let default_exchange = cfg_manager.default_exchange();
    let exchanges = cfg_manager.exchanges_id_names().to_owned();
    let exchange_symbol_table = cfg_manager
        .get_symbol_table(default_exchange)
        .expect("[main]: Failed to get symbol table for default exchange.");

    println!("{FN_NAME}: Creating a new QueryDBManager.");
    let db_config = cfg_manager.db_config();
    let mut db_query_manager = QueryDBManager::new(db_config.clone())
        .await
        .expect("[main]: Failed to create QueryDBManager instance.");

    // Get all symbols for the default exchange.
    let symbols = db_query_manager
        .get_all_symbols_with_ids(&exchange_symbol_table)
        .await
        .expect("[main]: Failed to get all symbols for SymbolManager.");

    println!("{FN_NAME}: Creating a new SymbolManager.");
    let mut symbol_manager = SymbolManager::new(symbols, exchanges)
        .expect("[main]: Failed to create SymbolManager instance.");

    let exchange_id = 1;
    let symbol_id = 2;
    let symbol_table_name = symbol_manager
        .get_symbol_table_name(exchange_id, symbol_id)
        .expect("[main]: Failed to get symbol table name");

    println!("{FN_NAME}: Load Data");
    let data = load_data::load_data(&mut db_query_manager, symbol_id, &symbol_table_name)
        .await
        .expect("[main]: Failed to load data.");

    println!("{FN_NAME}: Build Context");
    let context = build_context::build_time_data_context(&data, &TimeScale::Month, 250)
        .expect("[main]:  to build context");

    println!("Context HyperGraph Metrics:");
    println!("Edge Count: {}", context.edge_count());
    println!("Vertex Count: {}", context.node_count());

    println!("{FN_NAME}: Build Causal Model");
    let causaloid = model::get_main_causaloid(&context);
    let model = model::build_model(&context, &causaloid);

    println!("Causal Model:");
    println!("Model ID: {}", model.id());
    println!("Model Description: {}", model.description());
    println!();

    println!("{FN_NAME}: Build Client config for client ID: {CLIENT_ID}",);
    let client_config = MessageClientConfig::new(CLIENT_ID);

    println!("{FN_NAME}: Build QD Client",);
    let client = QDClient::new(CLIENT_ID, client_config.clone())
        .await
        .expect("basic_data_stream/main: Failed to create QD Gateway client");

    // println!("{FN_NAME}: Start the data handler",);
    // let data_topic = client_config.data_channel();
    // tokio::spawn(async move {
    //     if let Err(e) = handle_utils::handle_channel(&data_topic, handle_data_message).await {
    //         eprintln!("[QDClient/new]: Consumer connection error: {}", e);
    //     }
    // });

    println!("{FN_NAME}: Start the error handler");
    let err_topic = client_config.error_channel();
    tokio::spawn(async move {
        if let Err(e) =
            handle_utils::handle_channel(&err_topic, handle_error_utils::handle_error_message).await
        {
            eprintln!("[QDClient/new]: Consumer connection error: {}", e);
        }
    });

    println!("{FN_NAME}: Wait a moment to let the OHLCV data stream complete...");
    sleep(Duration::from_secs(2)).await;

    println!("{FN_NAME}: Closing client");
    client.close().await.expect("Failed to close client");
}

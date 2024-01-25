use client_utils::{handle_error_utils, handle_utils, print_utils};
use common::prelude::{ExchangeID, MessageClientConfig, ServiceID};
use config_manager::ConfigManager;
use db_query_manager::QueryDBManager;
use deep_causality::prelude::{ContextuableGraph, Identifiable, TimeScale};
use lib_inference::prelude::{build_context, data_handler, load_data, model};
use qd_client::QDClient;
use std::time::Duration;
use symbol_manager::SymbolManager;
use tokio::time::sleep;

const EXAMPLE: &'static str = "Causal Data Inference";

const FN_NAME: &'static str = "causal_data_inference/main";

const CLIENT_ID: u16 = 77;

const EXCHANGE_ID: ExchangeID = ExchangeID::Kraken;

//  Symbols
// const XBT_EUR: u16 = 202; // BTC in EUR ~80 million trades ~ 124 months
const JTO_EUR: u16 = 708; // JPY in EUR 2420 trades ~ 1 months

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

    let exchange_id = EXCHANGE_ID;
    let symbol_id = JTO_EUR;

    let symbol_table_name = symbol_manager
        .get_symbol_table_name(exchange_id as u16, symbol_id)
        .expect("[main]: Failed to get symbol table name");

    println!("{FN_NAME}: Load Data");
    let data = load_data::load_data(&mut db_query_manager, symbol_id, &symbol_table_name)
        .await
        .expect("[main]: Failed to load data.");

    let m_len = data.month_bars().len();
    println!("{FN_NAME}: Loaded Data for {m_len} months.");

    println!("{FN_NAME}: Build Context");
    let context = build_context::build_time_data_context(&data, &TimeScale::Month, 250)
        .expect("[main]:  to build context");

    println!();
    println!("Context HyperGraph Metrics:");
    println!("Edge Count: {}", context.edge_count());
    println!("Vertex Count: {}", context.node_count());

    println!("{FN_NAME}: Build Causal Model");
    let causaloid = model::get_main_causaloid(&context);
    let model = model::build_model(&context, &causaloid);

    println!();
    println!("Causal Model Information:");
    println!("Model ID: {}", model.id());
    println!("Model Description: {}", model.description());
    println!();

    println!("{FN_NAME}: Build Client config for client ID: {CLIENT_ID}",);
    let client_config = MessageClientConfig::new(CLIENT_ID);

    println!("{FN_NAME}: Build QD Client",);
    let client = QDClient::new(CLIENT_ID, client_config.clone())
        .await
        .expect("basic_data_stream/main: Failed to create QD Gateway client");

    println!("{FN_NAME}: Start the data handlers",);
    let data_topic = client_config.data_channel();
    tokio::spawn(async move {
        if let Err(e) =
            handle_utils::handle_channel(&data_topic, data_handler::handle_data_message).await
        {
            eprintln!("[QDClient/new]: Consumer connection error: {}", e);
        }
    });

    println!("{FN_NAME}: Start the error handlers");
    let err_topic = client_config.error_channel();
    tokio::spawn(async move {
        if let Err(e) =
            handle_utils::handle_channel(&err_topic, handle_error_utils::handle_error_message).await
        {
            eprintln!("[QDClient/new]: Consumer connection error: {}", e);
        }
    });

    println!("{FN_NAME}: Send start streaming message for symbol id: {symbol_id}",);
    client
        .start_trade_data(exchange_id, symbol_id)
        .await
        .expect("Failed to send start trade data message");

    println!("{FN_NAME}: Wait a moment to let the data stream complete...");
    sleep(Duration::from_secs(3)).await;

    println!("{FN_NAME}: Closing client");
    client.close().await.expect("Failed to close client");
}

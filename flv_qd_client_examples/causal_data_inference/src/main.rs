use crate::handle_data::MessageHandler;
use causal_model::context::build_context;
use causal_model::model::{build_main_causaloid, model_builder};
use client_utils::{data_utils, handle_error_utils, handle_utils, print_utils};
use common::prelude::{ExchangeID, MessageClientConfig, SampledDataBars, ServiceID};
use config_manager::ConfigManager;
use deep_causality::prelude::TimeScale;
use qd_client::QDClient;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

mod handle_data;

const FN_NAME: &str = "causal_data_inference/main";

const CLIENT_ID: u16 = 77;

const EXCHANGE_ID: ExchangeID = ExchangeID::Kraken;

const SYMBOL: &str = "pytheur"; // pytheur 11172 trades

///
/// The main function loads historical data, connects to a stream, handles errors,
/// builds a model, and processes streaming data through the model to make causal inferences.
/// The concurrent async tasks allow both data processing and error handling to happen simultaneously.
///
/// The main logic flow is:
/// * Prints the example header.
/// * Creates a ConfigManager to access configurations.
/// * Loads historical trade data for the specified symbol from the config.
/// * Prints a summary of the loaded data.
/// * Creates a MessageClientConfig for the client ID.
/// * Creates a QDClient using the config to connect to the data stream.
/// * Starts an async error handling task using the client config.
/// * Starts an async data handling task, passing the client config and historical data.
/// * Waits briefly to let data stream, then closes the client.
///
/// The error handling task listens on the error stream and logs any errors.
/// This avoids disrupting the main loop.
///
/// The data handling task builds a context and causal model using the historical data.
/// It spawns an async task to process each incoming data message through the model to run causal inference.
///
#[tokio::main]
async fn main() {
    print_utils::print_example_header("Causal Data Inference");

    println!("{FN_NAME}: Creating a new ConfigManager.");
    let cfg_manager = async { ConfigManager::new(ServiceID::Default) }.await;

    println!("{FN_NAME}: Load Data");
    let exchange_id = EXCHANGE_ID;
    let symbol_id = SYMBOL;
    let data = data_utils::load_data(&cfg_manager, symbol_id, exchange_id)
        .await
        .expect("[main]: Failed to load data.");

    print_data_summary(&data);

    println!("{FN_NAME}: Build Client config for client ID: {CLIENT_ID}",);
    let client_config = MessageClientConfig::new(CLIENT_ID);

    println!("{FN_NAME}: Build QD Client",);
    let client = QDClient::new(CLIENT_ID, client_config.clone())
        .await
        .expect("[main]: Failed to create QD Gateway client");

    println!("{FN_NAME}: Start the error handler");
    spawn_error_handler(client_config.clone()).await;

    println!("{FN_NAME}: Start the data handler");
    spawn_data_handler(client_config.clone(), data).await;

    // println!("{FN_NAME}: Send start streaming message for symbol id: {symbol_id}",);
    // client
    //     .start_trade_data(exchange_id, symbol_id)
    //     .await
    //     .expect("Failed to send start trade data message");

    println!("{FN_NAME}: Wait a moment to let the data stream complete...");
    sleep(Duration::from_secs(1)).await;

    println!("{FN_NAME}: Closing client");
    client.close().await.expect("Failed to close client");
}

/// Spawns an async task to handle errors from the Fluvio error stream.
///
/// This creates a separate async task that runs for the lifetime of the application.
/// It listens to the client's error channel for any errors sent from the main consumer loop.
///
/// Whenever an error is received on the channel, it will log the error to the console.
///
/// This avoids the main loop being disrupted by transient consumer errors.
///
/// # Arguments
///
/// * client_config: MessageClientConfig,
///
async fn spawn_error_handler(client_config: MessageClientConfig) {
    let err_topic = client_config.error_channel();
    tokio::spawn(async move {
        if let Err(e) =
            handle_utils::handle_channel(&err_topic, handle_error_utils::handle_error_message).await
        {
            eprintln!("{FN_NAME}: Consumer connection error: {}", e);
        }
    });
}

/// Spawns an async task to handle data messages from the Fluvio data stream.
///
/// This creates a separate async task that runs for the lifetime of the application.
///
/// The messages are passed to the `handle_data_message` function to process the data.
///
/// # Arguments
///
/// * `model` - The causal model to use for processing the data
async fn spawn_data_handler(client_config: MessageClientConfig, data: SampledDataBars) {
    let data_topic = client_config.data_channel();

    // Spawn the task to handle the data messages.
    tokio::spawn(async move {
        // We have to build a context for the model inside the async block
        // so that the task owns all internal references to the data, context, and causaloid.
        // This is because the task will be dropped when the async block ends and that way,
        // the data, context, and causaloid will live as long as the task is alive.
        //
        // This is necessary because the DeepCausality crate uses references with lifetimes
        // internally to store the context, which may grow large if the dataset is large.
        //
        let context = build_context::build_time_data_context(&data, &TimeScale::Month, 10)
            .expect("[main]: Failed to build context");
        let causaloid = build_main_causaloid::build_main_causaloid(&context);
        let model = Arc::new(model_builder::build_causal_model(&context, causaloid));
        let handler = MessageHandler::new(data_topic, model);

        if let Err(e) = handler.run_inference().await {
            eprintln!("{FN_NAME}: Data processing error: {}", e);
        }
    });
}

fn print_data_summary(data: &SampledDataBars) {
    let y_len = data.year_bars().len();
    println!("{FN_NAME}: Loaded Data for {y_len} year(s).");

    let m_len = data.month_bars().len();
    println!("{FN_NAME}: Loaded Data for {m_len} month(s).");
}

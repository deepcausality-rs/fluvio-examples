use causal_model::context::build_context;
use causal_model::model::causaloid_range_month;
use causal_model::prelude::{CustomContext, TimeIndexable};
use client_utils::data_utils;
use common::prelude::{ExchangeID, ServiceID};
use config_manager::ConfigManager;
use deep_causality::prelude::{ContextuableGraph, Identifiable, TimeScale};
use std::env;

async fn get_context() -> CustomContext<'static> {
    env::set_var("ENV", "Local");

    let cfg_manager = async { ConfigManager::new(ServiceID::Default) }.await;
    let exchange_id = ExchangeID::Kraken;
    let symbol_id = 708; // JPY in EUR 2420 trades ~ 1 months
    let data = data_utils::load_data(&cfg_manager, symbol_id, exchange_id)
        .await
        .expect("[get_context]: Failed to load data.");

    let context = build_context::build_time_data_context(&data, &TimeScale::Month, 10)
        .expect("[get_context]: Failed to build context");

    context
}

#[tokio::test]
async fn test_get_month_causaloid() {
    let context = get_context().await;
    let causaloid = causaloid_range_month::get_month_causaloid(&context, 1);

    assert_eq!(causaloid.id(), 1);
}

#[tokio::test]
async fn test_current_month_index() {
    let context = get_context().await;
    let causaloid = causaloid_range_month::get_month_causaloid(&context, 1);

    assert_eq!(causaloid.id(), 1);

    let current_month_index = causaloid.context().unwrap().get_current_month_index();
    assert!(current_month_index.is_some());
}

#[tokio::test]
async fn test_current_month_node() {
    let context = get_context().await;
    let causaloid = causaloid_range_month::get_month_causaloid(&context, 1);

    assert_eq!(causaloid.id(), 1);

    let current_month_index = causaloid.context().unwrap().get_current_month_index();
    assert!(current_month_index.is_some());

    let current_month_index = current_month_index.unwrap();
    let month_node = causaloid.context().unwrap().get_node(*current_month_index);
    assert!(month_node.is_some());
}

#[tokio::test]
async fn test_previous_month_index() {
    let context = get_context().await;
    let causaloid = causaloid_range_month::get_month_causaloid(&context, 1);

    assert_eq!(causaloid.id(), 1);

    //
    // Current data set ( 708 / JPY) has only 1 month of data.
    //
    // let previous_month_index = causaloid.context().unwrap().get_previous_month_index();
    // assert!(previous_month_index.is_some());
    //
    // let previous_month_index = previous_month_index.unwrap();
    // let month_node = causaloid.context().unwrap().get_node(*previous_month_index);
    // assert!(month_node.is_some());
}

#[tokio::test]
async fn test_previous_month_node() {
    let context = get_context().await;
    let causaloid = causaloid_range_month::get_month_causaloid(&context, 1);

    assert_eq!(causaloid.id(), 1);
    //
    // Add tests once we have more data.
}

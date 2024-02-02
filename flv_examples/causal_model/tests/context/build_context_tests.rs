use causal_model::context::build_context;
use client_utils::data_utils;
use common::prelude::{ExchangeID, SampledDataBars, ServiceID};
use config_manager::ConfigManager;
use deep_causality::prelude::{ContextuableGraph, Identifiable, TimeScale};
use std::env;

const JTO_EUR: &str = "jtoeur"; // JPY in EUR 2420 trades ~ 1 months

async fn get_data() -> SampledDataBars {
    env::set_var("ENV", "Local");

    let cfg_manager = async { ConfigManager::new(ServiceID::Default) }.await;
    let exchange_id = ExchangeID::Kraken;
    let symbol_id = JTO_EUR;

    data_utils::load_data(&cfg_manager, symbol_id, exchange_id)
        .await
        .expect("[get_data]: Failed to load data.")
}

#[tokio::test]
async fn test_build_time_data_context() {
    let data = get_data().await;
    let time_scale = TimeScale::Year;
    let capacity = 10;

    let result = build_context::build_time_data_context(&data, &time_scale, capacity);
    assert!(result.is_ok());

    let context = result.unwrap();
    // Three nodes in total: root, year, month
    assert_eq!(context.node_count(), 3);

    let root = context
        .get_node(0)
        .expect("[test_build_time_data_context]: Failed to get root node.");
    assert_eq!(root.id(), 1);

    let year = context
        .get_node(1)
        .expect("[test_build_time_data_context]: Failed to get year node.");
    assert_eq!(year.id(), 2);

    let month = context
        .get_node(2)
        .expect("[test_build_time_data_context]: Failed to get month node.");
    assert_eq!(month.id(), 3);
}

use causal_model::context::build_context;
use causal_model::model::causaloid_range_year;
use causal_model::prelude::{CustomContext, TimeIndexable};
use client_utils::data_utils;
use common::prelude::{ExchangeID, ServiceID};
use config_manager::ConfigManager;
use deep_causality::prelude::{Contextuable, ContextuableGraph, Identifiable, TimeScale};
use std::env;

async fn get_context() -> CustomContext<'static> {
    env::set_var("ENV", "Local");

    let cfg_manager = async { ConfigManager::new(ServiceID::Default) }.await;
    let exchange_id = ExchangeID::Kraken;
    let symbol_id = "jtoeur"; //2420 trades ~ 1 months
    let data = data_utils::load_data(&cfg_manager, symbol_id, exchange_id)
        .await
        .expect("[get_context]: Failed to load data.");

    let context = build_context::build_time_data_context(&data, &TimeScale::Month, 10)
        .expect("[get_context]: Failed to build context");

    context
}

#[tokio::test]
async fn test_get_year_causaloid() {
    let context = get_context().await;

    let causaloid = causaloid_range_year::get_year_causaloid(&context, 1);

    assert_eq!(causaloid.id(), 1);
}

#[tokio::test]
async fn test_current_year_index() {
    let context = get_context().await;

    let causaloid = causaloid_range_year::get_year_causaloid(&context, 1);
    assert_eq!(causaloid.id(), 1);

    let current_year_index = causaloid.context().unwrap().get_current_year_index();
    assert!(current_year_index.is_some());
}

#[tokio::test]
async fn test_current_year_node() {
    let context = get_context().await;

    let causaloid = causaloid_range_year::get_year_causaloid(&context, 1);
    assert_eq!(causaloid.id(), 1);

    let current_year_index = causaloid.context().unwrap().get_current_year_index();
    assert!(current_year_index.is_some());

    let current_year_index = current_year_index.unwrap();
    let year_node = causaloid.context().unwrap().get_node(*current_year_index);
    assert!(year_node.is_some());

    let year_node = year_node.unwrap();
    let data = year_node.vertex_type().dataoid();
    assert!(data.is_some());
}

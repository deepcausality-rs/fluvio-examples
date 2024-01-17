use client_utils::sbe_utils::{
    encode_login_message, encode_logout_message, encode_start_data_message,
};
use common::prelude::{ExchangeID, SymbolID};
use sbe_messages::prelude::DataType;

#[tokio::test]
async fn test_encode_login_message() {
    let client_id = 1;
    let result = encode_login_message(client_id).await;
    assert!(result.is_ok());

    let encoded = result.unwrap();
    assert!(!encoded.is_empty());

    let expected: Vec<u8> = vec![4, 0, 101, 0, 1, 0, 1, 0, 101, 0, 1, 0];
    let actual = encoded;
    assert_eq!(expected, actual);
}

#[tokio::test]
async fn test_encode_logout_message() {
    let client_id = 100;
    let result = encode_logout_message(client_id).await;
    assert!(result.is_ok());

    let encoded = result.unwrap();
    assert!(!encoded.is_empty());

    let expected: Vec<u8> = vec![4, 0, 102, 0, 1, 0, 1, 0, 102, 0, 100, 0];
    let actual = encoded;
    assert_eq!(expected, actual);
}

#[tokio::test]
async fn test_encode_start_data_message() {
    let client_id = 1;
    let exchange_id = ExchangeID::BinanceSpot;
    let symbol_id = SymbolID::BTCUSD as u16;
    let data_type = DataType::TradeData;

    let result = encode_start_data_message(client_id, exchange_id, symbol_id, data_type).await;
    assert!(result.is_ok());

    let encoded = result.unwrap();
    assert!(!encoded.is_empty());

    let expected: Vec<u8> = vec![8, 0, 201, 0, 1, 0, 1, 0, 201, 0, 1, 0, 1, 1, 0, 1];
    let actual = encoded;
    assert_eq!(expected, actual);
}

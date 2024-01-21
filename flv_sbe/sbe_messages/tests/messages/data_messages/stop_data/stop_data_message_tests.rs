use common::prelude::ExchangeID;
use sbe_messages::prelude::{DataType, MessageType, StopDataMessage};

fn get_message() -> StopDataMessage {
    let client_id = 1;
    let exchange_id = ExchangeID::Kraken;
    let symbol_id = 1;
    let data_type = DataType::TradeData;

    StopDataMessage::new(client_id, exchange_id, symbol_id, data_type)
}

#[test]
fn test_new() {
    let message = get_message();

    let exchange_id = ExchangeID::Kraken;
    let symbol_id = 1;
    let data_type = DataType::TradeData;

    assert_eq!(message.message_type(), &MessageType::StopData);
    assert_eq!(message.client_id(), &1);
    assert_eq!(message.exchange_id(), &exchange_id);
    assert_eq!(message.symbol_id(), &symbol_id);
    assert_eq!(message.data_type_id(), &data_type);
}

#[test]
fn test_encode() {
    let message = get_message();

    let exchange_id = ExchangeID::Kraken;
    let symbol_id = 1;
    let data_type = DataType::TradeData;

    assert_eq!(message.message_type(), &MessageType::StopData);
    assert_eq!(message.client_id(), &1);
    assert_eq!(message.exchange_id(), &exchange_id);
    assert_eq!(message.symbol_id(), &symbol_id);
    assert_eq!(message.data_type_id(), &data_type);

    let enc = message.encode();
    assert!(enc.is_ok());

    let (limit, buffer) = enc.unwrap();
    assert_eq!(limit, 16);

    let expected: Vec<u8> = vec![8, 0, 202, 0, 1, 0, 1, 0, 202, 0, 1, 0, 1, 1, 0, 1];
    let actual = buffer;
    assert_eq!(expected, actual);
}

#[test]
fn test_decode() {
    let encoded: Vec<u8> = vec![8, 0, 202, 0, 1, 0, 1, 0, 202, 0, 1, 0, 1, 1, 0, 1];
    let buffer = encoded.as_slice();

    let message = StopDataMessage::from(buffer);

    let exchange_id = ExchangeID::Kraken;
    let symbol_id = 1;
    let data_type = DataType::TradeData;

    assert_eq!(message.message_type(), &MessageType::StopData);
    assert_eq!(message.client_id(), &1);
    assert_eq!(message.exchange_id(), &exchange_id);
    assert_eq!(message.symbol_id(), &symbol_id);
    assert_eq!(message.data_type_id(), &data_type);
}

#[test]
fn test_message_type() {
    let message = get_message();

    assert_eq!(message.message_type(), &MessageType::StopData);
}

#[test]
fn test_message_client_id() {
    let message = get_message();

    assert_eq!(message.client_id(), &1);
}

#[test]
fn test_exchange_id() {
    let message = get_message();

    let exchange_id = ExchangeID::Kraken;

    assert_eq!(message.exchange_id(), &exchange_id);
}

#[test]
fn test_symbol_id() {
    let message = get_message();

    let symbol_id = 1;

    assert_eq!(message.symbol_id(), &symbol_id);
}

#[test]
fn test_display() {
    let message = get_message();

    let expected = "StopDataMessage[message_type: StopData, client_id: 1, exchange_id: Kraken, symbol_id: 1, data_type: TradeData]";
    let actual = format!("{}", message);
    assert_eq!(expected, actual);
}

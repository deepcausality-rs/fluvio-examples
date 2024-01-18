use common::prelude::{ExchangeID, SymbolID, TimeResolution};
use sbe_messages::prelude::{DataType, MessageType, StartDataMessage};

fn get_message() -> StartDataMessage {
    let client_id = 1;
    let exchange_id = ExchangeID::BinanceSpot;
    let symbol_id = SymbolID::BTCUSD as u16;
    let data_type = DataType::TradeData;
    let time_resolution = TimeResolution::NoValue; // TimeResolution only applies to OHLCV data type
    StartDataMessage::new(
        client_id,
        exchange_id,
        symbol_id,
        time_resolution,
        data_type,
    )
}

#[test]
fn test_new() {
    let message = get_message();

    let exchange_id = ExchangeID::BinanceSpot;
    let symbol_id = SymbolID::BTCUSD as u16;
    let data_type = DataType::TradeData;
    let time_resolution = TimeResolution::NoValue;

    assert_eq!(message.message_type(), &MessageType::StartData);
    assert_eq!(message.client_id(), &1);
    assert_eq!(message.exchange_id(), &exchange_id);
    assert_eq!(message.symbol_id(), &symbol_id);
    assert_eq!(message.time_resolution(), &time_resolution);
    assert_eq!(message.data_type_id(), &data_type);
}

#[test]
fn test_encode() {
    let message = get_message();

    let exchange_id = ExchangeID::BinanceSpot;
    let symbol_id = SymbolID::BTCUSD as u16;
    let time_resolution = TimeResolution::NoValue;
    let data_type = DataType::TradeData;

    assert_eq!(message.message_type(), &MessageType::StartData);
    assert_eq!(message.client_id(), &1);
    assert_eq!(message.exchange_id(), &exchange_id);
    assert_eq!(message.symbol_id(), &symbol_id);
    assert_eq!(message.time_resolution(), &time_resolution);
    assert_eq!(message.data_type_id(), &data_type);

    let enc = message.encode();
    assert!(enc.is_ok());

    let (limit, buffer) = enc.unwrap();
    assert_eq!(limit, 17);

    let expected: Vec<u8> = vec![9, 0, 201, 0, 1, 0, 1, 0, 201, 0, 1, 0, 1, 1, 0, 0, 1];
    let actual = buffer;

    assert_eq!(expected, actual);
}

#[test]
fn test_decode() {
    let encoded: Vec<u8> = vec![9, 0, 201, 0, 1, 0, 1, 0, 201, 0, 1, 0, 1, 1, 0, 0, 1];
    let buffer = encoded.as_slice();

    let message = StartDataMessage::from(buffer);

    let exchange_id = ExchangeID::BinanceSpot;
    let symbol_id = SymbolID::BTCUSD as u16;
    let time_resolution = TimeResolution::NoValue;
    let data_type = DataType::TradeData;

    assert_eq!(message.message_type(), &MessageType::StartData);
    assert_eq!(message.client_id(), &1);
    assert_eq!(message.exchange_id(), &exchange_id);
    assert_eq!(message.symbol_id(), &symbol_id);
    assert_eq!(message.time_resolution(), &time_resolution);
    assert_eq!(message.data_type_id(), &data_type);
}

#[test]
fn test_message_type() {
    let message = get_message();

    assert_eq!(message.message_type(), &MessageType::StartData);
}

#[test]
fn test_message_client_id() {
    let message = get_message();

    assert_eq!(message.client_id(), &1);
}

#[test]
fn test_exchange_id() {
    let message = get_message();
    let exchange_id = ExchangeID::BinanceSpot;

    assert_eq!(message.exchange_id(), &exchange_id);
}

#[test]
fn test_symbol_id() {
    let message = get_message();
    let symbol_id = SymbolID::BTCUSD as u16;

    assert_eq!(message.symbol_id(), &symbol_id);
}

#[test]
fn test_time_resolution() {
    let message = get_message();
    let time_resolution = TimeResolution::NoValue;

    assert_eq!(message.time_resolution(), &time_resolution);
}

#[test]
fn test_display() {
    let message = get_message();

    let expected = format!(
        "StartDataMessage[message_type: {}, client_id: {}, exchange_id: {}, symbol_id: {} time_resolution: {} data_type: {}]",
        message.message_type(),
        message.client_id(),
        message.exchange_id(),
        message.symbol_id(),
        message.time_resolution(),
        message.data_type_id()
    );

    let actual = format!("{}", message);
    assert_eq!(expected, actual);
}

use common::prelude::{ExchangeID, SymbolID};
use sbe_messages::prelude::{MessageType, StartDataMessage};

#[test]
fn test_new() {
    let client_id = 1;
    let exchange_id = ExchangeID::BinanceSpot;
    let symbol_id = SymbolID::BTCUSD as u16;
    let message = StartDataMessage::new(client_id, exchange_id, symbol_id);

    assert_eq!(message.message_type(), &MessageType::StartData);
    assert_eq!(message.client_id(), &1);
    assert_eq!(message.exchange_id(), &ExchangeID::BinanceSpot);
    assert_eq!(message.symbol_id(), &symbol_id);
}

#[test]
fn test_encode() {
    let client_id = 1;
    let exchange_id = ExchangeID::BinanceSpot;
    let symbol_id = SymbolID::BTCUSD as u16;
    let message = StartDataMessage::new(client_id, exchange_id, symbol_id);

    assert_eq!(message.message_type(), &MessageType::StartData);
    assert_eq!(message.client_id(), &1);
    assert_eq!(message.exchange_id(), &ExchangeID::BinanceSpot);
    assert_eq!(message.symbol_id(), &symbol_id);

    let enc = message.encode();
    assert!(enc.is_ok());

    let (limit, buffer) = enc.unwrap();
    assert_eq!(limit, 15);

    let expected: Vec<u8> = vec![7, 0, 201, 0, 1, 0, 1, 0, 201, 0, 1, 0, 1, 1, 0];
    let actual = buffer;

    assert_eq!(expected, actual);
}

#[test]
fn test_decode() {
    let symbol_id = SymbolID::BTCUSD as u16;

    let encoded: Vec<u8> = vec![7, 0, 201, 0, 1, 0, 1, 0, 201, 0, 1, 0, 1, 1, 0];
    let buffer = encoded.as_slice();

    let message = StartDataMessage::from(buffer);
    assert_eq!(message.message_type(), &MessageType::StartData);

    assert_eq!(message.client_id(), &1);

    assert_eq!(message.exchange_id(), &ExchangeID::BinanceSpot);
    assert_eq!(message.symbol_id(), &symbol_id);
}

#[test]
fn test_message_type() {
    let client_id = 1;
    let exchange_id = ExchangeID::BinanceSpot;
    let symbol_id = SymbolID::BTCUSD as u16;
    let message = StartDataMessage::new(client_id, exchange_id, symbol_id);

    assert_eq!(message.message_type(), &MessageType::StartData);
}

#[test]
fn test_message_client_id() {
    let client_id = 1;
    let exchange_id = ExchangeID::BinanceSpot;
    let symbol_id = SymbolID::BTCUSD as u16;
    let message = StartDataMessage::new(client_id, exchange_id, symbol_id);

    assert_eq!(message.client_id(), &1);
}

#[test]
fn test_exchange_id() {
    let client_id = 1;
    let exchange_id = ExchangeID::BinanceSpot;
    let symbol_id = SymbolID::BTCUSD as u16;
    let message = StartDataMessage::new(client_id, exchange_id, symbol_id);

    assert_eq!(message.exchange_id(), &ExchangeID::BinanceSpot);
}

#[test]
fn test_symbol_id() {
    let client_id = 1;
    let exchange_id = ExchangeID::BinanceSpot;
    let symbol_id = SymbolID::BTCUSD as u16;
    let message = StartDataMessage::new(client_id, exchange_id, symbol_id);

    assert_eq!(message.symbol_id(), &symbol_id);
}

#[test]
fn test_display() {
    let client_id = 1;
    let exchange_id = ExchangeID::BinanceSpot;
    let symbol_id = SymbolID::BTCUSD as u16;
    let message = StartDataMessage::new(client_id, exchange_id, symbol_id);

    let expected = "StartDataMessage[message_type: StartData, client_id: 1, exchange_id: BinanceSpot, symbol_id: 1]";
    let actual = format!("{}", message);
    assert_eq!(expected, actual);
}

use common::prelude::ExchangeID;
use sbe_messages::prelude::{MessageType, StopAllDataMessage};

#[test]
fn test_new() {
    let client_id = 1;
    let exchange_id = ExchangeID::BinanceSpot;
    let message = StopAllDataMessage::new(client_id, exchange_id);

    assert_eq!(message.message_type(), &MessageType::StopAllData);
    assert_eq!(message.client_id(), &1);
    assert_eq!(message.exchange_id(), &ExchangeID::BinanceSpot);
}

#[test]
fn test_encode() {
    let client_id = 1;
    let exchange_id = ExchangeID::BinanceSpot;
    let message = StopAllDataMessage::new(client_id, exchange_id);

    assert_eq!(message.message_type(), &MessageType::StopAllData);
    assert_eq!(message.client_id(), &1);
    assert_eq!(message.exchange_id(), &ExchangeID::BinanceSpot);

    let enc = message.encode();
    assert!(enc.is_ok());

    let (limit, buffer) = enc.unwrap();
    assert_eq!(limit, 12);

    let expected: Vec<u8> = vec![4, 0, 5, 0, 1, 0, 1, 0, 5, 1, 0, 1];
    let actual = buffer;

    assert_eq!(expected, actual);
}

#[test]
fn test_decode() {
    let encoded: Vec<u8> = vec![4, 0, 5, 0, 1, 0, 1, 0, 5, 1, 0, 1];
    let buffer = encoded.as_slice();

    let message = StopAllDataMessage::from(buffer);
    assert_eq!(message.message_type(), &MessageType::StopAllData);
    assert_eq!(message.client_id(), &1);
    assert_eq!(message.exchange_id(), &ExchangeID::BinanceSpot);
}

#[test]
fn test_message_type() {
    let client_id = 1;
    let exchange_id = ExchangeID::BinanceSpot;
    let message = StopAllDataMessage::new(client_id, exchange_id);

    assert_eq!(message.message_type(), &MessageType::StopAllData);
}

#[test]
fn test_message_client_id() {
    let client_id = 1;
    let exchange_id = ExchangeID::BinanceSpot;
    let message = StopAllDataMessage::new(client_id, exchange_id);

    assert_eq!(message.client_id(), &1);
}

#[test]
fn test_exchange_id() {
    let client_id = 1;
    let exchange_id = ExchangeID::BinanceSpot;
    let message = StopAllDataMessage::new(client_id, exchange_id);

    assert_eq!(message.exchange_id(), &ExchangeID::BinanceSpot);
}

#[test]
fn test_display() {
    let client_id = 1;
    let exchange_id = ExchangeID::BinanceSpot;
    let message = StopAllDataMessage::new(client_id, exchange_id);

    let expected =
        "StopAllDataMessage[message_type: StopAllData, client_id: 1, exchange_id: BinanceSpot]";
    let actual = format!("{}", message);
    assert_eq!(expected, actual);
}

use common::prelude::{ExchangeID, HostEndpoint};
use symdb_client::SymdbClient;

fn get_config() -> HostEndpoint {
    HostEndpoint::new("0.0.0.0".to_string(), 7070)
}

//
// Requires that the SYMDB server is running on localhost:7070
// Start the server with:
//
// cargo run --bin symdb_server
//

#[tokio::test]
async fn test_lookup_exchange_name() {
    let config = get_config();
    let mut client = SymdbClient::new(config).await.unwrap();
    let result = client.lookup_exchange_name(ExchangeID::Kraken).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "kraken");
}

#[tokio::test]
async fn test_lookup_exchange_name_invalid() {
    let config = get_config();
    let mut client = SymdbClient::new(config).await.unwrap();

    let result = client.lookup_exchange_name(ExchangeID::NullVal).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_lookup_symbol() {
    let config = get_config();
    let mut client = SymdbClient::new(config).await.unwrap();

    let result = client.lookup_symbol(ExchangeID::Kraken, 42).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_lookup_symbol_invalid() {
    let config = get_config();
    let mut client = SymdbClient::new(config).await.unwrap();

    let result = client.lookup_symbol(ExchangeID::Kraken, 9999).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_lookup_symbol_id() {
    let config = get_config();
    let mut client = SymdbClient::new(config).await.unwrap();

    let symbol = "ethaed".to_string();
    let result = client.lookup_symbol_id(ExchangeID::Kraken, symbol).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_lookup_symbol_id_invalid() {
    let config = get_config();
    let mut client = SymdbClient::new(config).await.unwrap();

    let symbol = "InvalidSymbol".to_string();
    let result = client.lookup_symbol_id(ExchangeID::Kraken, symbol).await;
    assert!(result.is_err());
}

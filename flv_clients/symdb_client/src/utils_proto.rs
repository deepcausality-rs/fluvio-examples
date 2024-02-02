use common::prelude::ExchangeID;
use proto::binding::{LookupExchangeNameRequest, LookupSymbolIdRequest, LookupSymbolRequest};

/// Creates a tonic::Request for the GetExchange RPC method.
///
/// # Arguments
///
/// * `exchange_id` - The u32 ID of the exchange to look up
///
/// # Returns
///
/// Returns a tonic::Request with the GetExchange RPC request populated
/// with the provided exchange ID.
///
pub(crate) fn get_exchange_request(exchange_id: ExchangeID) -> LookupExchangeNameRequest {
    let exchange_id = exchange_id as i32;
    LookupExchangeNameRequest { exchange_id }
}

/// Creates a tonic::Request for the GetSymbol RPC method.
///
/// # Arguments
///
/// * `symbol_id` - The u32 ID of the symbol to look up
///
/// # Returns
///
/// Returns a tonic::Request with the GetSymbol RPC request populated
/// with the provided symbol ID.
///
pub(crate) fn get_symbol_request(exchange_id: ExchangeID, symbol_id: u16) -> LookupSymbolRequest {
    let exchange_id = exchange_id as i32;
    let symbol_id = symbol_id as i32;

    LookupSymbolRequest {
        exchange_id,
        symbol_id,
    }
}

/// Creates a tonic::Request for the GetSymbolId RPC method.
///
/// # Arguments
///
/// * `symbol` - The symbol string to look up
///
/// # Returns
///
/// Returns a tonic::Request with the GetSymbolId RPC request populated
/// with the provided symbol string.
///
pub(crate) fn get_symbol_id_request(
    exchange_id: ExchangeID,
    symbol: String,
) -> LookupSymbolIdRequest {
    let exchange_id = exchange_id as i32;
    LookupSymbolIdRequest {
        exchange_id,
        symbol,
    }
}

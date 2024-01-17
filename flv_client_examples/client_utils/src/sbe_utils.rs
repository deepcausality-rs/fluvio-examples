use common::prelude::ExchangeID;
use sbe_messages::errors::SbeEncodeError;
use sbe_messages::prelude::{ClientLoginMessage, ClientLogoutMessage, DataType, StartDataMessage};

/// Encodes a ClientLoginMessage into a byte vector.
///
/// # Arguments
///
/// * `client_id` - The ID of the client sending the login message.
///
/// # Returns
///
/// Returns a Result containing the encoded message bytes on success, or an error on failure.
///
pub async fn encode_login_message(client_id: u16) -> Result<Vec<u8>, SbeEncodeError> {
    let message = ClientLoginMessage::new(client_id);
    let enc = message.encode();
    return match enc {
        Ok((_, buff)) => Ok(buff),
        Err(err) => Err(err),
    };
}

/// Encodes a ClientLogoutMessage into a byte vector.
///
/// # Arguments
///
/// * `client_id` - The ID of the client sending the logout message.
///
/// # Returns
///
/// Returns a Result containing the encoded message bytes on success, or an error on failure.
///
pub async fn encode_logout_message(client_id: u16) -> Result<Vec<u8>, SbeEncodeError> {
    let message = ClientLogoutMessage::new(client_id);
    let enc = message.encode();
    return match enc {
        Ok((_, buff)) => Ok(buff),
        Err(err) => Err(err),
    };
}

/// Encodes a StartDataMessage into a byte vector.
///
/// # Arguments
///
/// * `client_id` - The ID of the client sending the message.
/// * `exchange_id` - The exchange ID for the requested data.
/// * `symbol_id` - The symbol ID for the requested data.
/// * `data_type_id` - The data type being requested.
///
/// # Returns
///
/// Returns a Result containing the encoded message bytes on success, or an error on failure.
///
pub async fn encode_start_data_message(
    client_id: u16,
    exchange_id: ExchangeID,
    symbol_id: u16,
    data_type_id: DataType,
) -> Result<Vec<u8>, SbeEncodeError> {
    let message = StartDataMessage::new(client_id, exchange_id, symbol_id, data_type_id);
    let enc = message.encode();
    return match enc {
        Ok((_, buff)) => Ok(buff),
        Err(err) => Err(err),
    };
}

use sbe_messages::prelude::{
    ClientErrorMessage, ClientErrorType, DataErrorMessage, DataErrorType, MessageType,
};
use std::error::Error;

/// Handles error messages received from the gateway.
///
/// Takes the raw message bytes as parameter.
///
/// Gets the message type from the third byte.
///
/// Matches on message type:
///
/// - MessageType::ClientError:
///   Calls handle_client_error to handle client errors.
///
/// - MessageType::DataError:
///   Calls handle_data_error to handle data errors.
///
/// - Other MessageTypes:
///   Ignores the message.
///
/// Returns a Result with no value if successful, otherwise an error.
///
pub fn handle_error_message(message: Vec<u8>) -> Result<(), Box<dyn Error + Send>> {
    // The third byte of the buffer is always the message type.
    let message_type = MessageType::from(message[2] as u16);

    match message_type {
        // Handle client errors
        MessageType::ClientError => {
            handle_client_error(message)?;
        }
        // Handle data errors
        MessageType::DataError => {
            handle_data_error(message)?;
        }
        // Ignore any other message types
        _ => {}
    }

    Ok(())
}

/// Handles ClientErrorMessages sent from the gateway.
///
/// It takes a raw_event byte vector containing the serialized ClientErrorMessage as parameter.
///
/// It deserializes the raw_event into a ClientErrorMessage and extracts the client_error_type.
///
/// It then matches on the different ClientErrorTypes:
///
/// - ClientErrorType::ClientAlreadyLoggedIn:
///   The gateway sends this error when the client already logged in.
///   This is a normal behavior, and can be ignored.
///
/// - ClientErrorType::ClientLogInError:
///   The gateway sends this error when the client failed to log in.
///   This means, the client cannot connect to the gateway and thus must
///   be restarted, and try again most likely with a different client id.
///   This is a fatal error, and the client must be restarted.
///
/// - ClientErrorType::ClientNotLoggedIn:
///   The gateway sends this error when the client is not logged in.
///   This means, the client cannot connect to the gateway and thus must login first.
///   This is a fatal error, and the client must try to login again probably with a different client id.
///
/// - ClientErrorType::ClientLogOutError:
///   The gateway sends this error when the client failed to log out.
///   This is exceptional rare, and needs to be investigated.
///
/// - ClientErrorType::UnknownClientError:
///   The gateway sends this error in all other cases or when the origin of the error is unknown.
///   Assume something went totally wrong, restart the client,
///   and if the problem persists, investigate the gateway.
///
/// It returns a Result with no value if successful, otherwise an error.
///
fn handle_client_error(raw_event: Vec<u8>) -> Result<(), Box<dyn Error + Send>> {
    let buffer = raw_event.as_slice();
    let client_error = ClientErrorMessage::from(buffer);
    let client_error_type = client_error.client_error_type();

    match client_error_type {
        ClientErrorType::ClientAlreadyLoggedIn => {
            // The gateway sends this error when the client already logged in.
            // This is a normal behavior, and can be ignored.
            println!("ClientAlreadyLoggedIn");
        }
        ClientErrorType::ClientLogInError => {
            // The gateway sends this error when the client failed to log in.
            // This means, the client cannot connect to the gateway and thus must
            // be restarted, and try again most likely with a different client id.
            println!("ClientLogInError");
            // This is a fatal error, and the client must be restarted.
        }
        ClientErrorType::ClientNotLoggedIn => {
            // The gateway sends this error when the client is not logged in.
            // This means, the client cannot connect to the gateway and thus must login first.
            println!("ClientNotLoggedIn");
            // This is a fatal error, and the client must try to login again probably with a different client id.
        }
        ClientErrorType::ClientLogOutError => {
            // The gateway sends this error when the client failed to log out.
            // This is exceptional rare, and needs to be investigated.
            println!("ClientLogOutError");
        }
        ClientErrorType::UnknownClientError => {
            // The gateway sends this error in all other cases or when the origin of the error is unknown.
            // Assume something went totally wrong, restart the client,
            // and if the problem persists, investigate the gateway.
            println!("UnknownClientError");
        }
    }

    Ok(())
}

/// The handle_data_error function handles DataErrorMessages sent from the gateway.
///
/// It takes a msg byte vector containing the serialized DataErrorMessage as parameter.
///
/// It deserializes the msg into a DataErrorMessage and extracts the data_error_type.
///
/// It then matches on the different DataErrorTypes:
///
/// - DataErrorType::DataTypeNotKnownError:
///   The gateway will send this error if the client requests data for a type that is not known.
///
/// - DataErrorType::DataUnavailableError:
///   The gateway will send this error if the client requests data that is not available.
///
/// - DataErrorType::DataEncodingError:
///   The gateway will send this error if it fails to encode the data.
///
/// - DataErrorType::DataTableNotFound:
///   The gateway will send this error if the client requests data from a table that does not exist.
///
/// - DataErrorType::DataSendError:
///   The gateway will send this error if it somehow fails to send the data to the client.
///
/// - DataErrorType::DataChannelError:
///   The gateway will send this error if no connection to the client data channel exists.
///
/// - DataErrorType::UnknownDataError:
///   The gateway will send this error in all other cases where it cannot determine the error cause.
///
/// It returns a Result with no value if successful, otherwise an error.
///
fn handle_data_error(msg: Vec<u8>) -> Result<(), Box<dyn Error + Send>> {
    let buffer = msg.as_slice();
    let data_error = DataErrorMessage::from(buffer);
    let data_error_type = data_error.data_error_type();

    match data_error_type {
        DataErrorType::DataTypeNotKnownError => {
            // The gateway will send this error if the client requests data for a type that is not known
            println!("DataTypeNotKnownError");
        }
        DataErrorType::DataUnavailableError => {
            // The gateway will send this error if the client requests data that is not available
            println!("DataUnavailableError");
        }
        DataErrorType::DataEncodingError => {
            // The gateway will send this error if it fails to encode the data
            println!("DataEncodingError");
        }
        DataErrorType::DataTableNotFound => {
            // The gateway will send this error if the client requests data from a table that does not exist
            println!("DataTableNotFound");
        }
        DataErrorType::DataSendError => {
            // The gateway will send this error if it somehow fails to send the data to the client
            println!("DataSendError");
        }
        DataErrorType::DataChannelError => {
            // The gateway will send this error if no connection to the client data channel exists
            println!("DataChannelError");
        }
        DataErrorType::UnknownDataError => {
            // The gateway will send this error in all other cases where it cannot determine the error cause
            println!("UnknownDataError");
        }
    }

    Ok(())
}

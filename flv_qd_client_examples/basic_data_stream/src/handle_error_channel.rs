use common::prelude::MessageClientConfig;
use fluvio::Offset;
use futures::StreamExt;
use sbe_messages::prelude::{
    ClientErrorMessage, ClientErrorType, DataErrorMessage, DataErrorType, MessageType,
};
use std::error::Error;

pub(crate) async fn handle_error_channel(
    client_config: MessageClientConfig,
) -> Result<(), Box<dyn Error + Send>> {
    let error_topic = client_config.data_channel();

    let consumer = fluvio::consumer(error_topic, 0)
        .await
        .expect("Failed to create a consumer for data topic");

    let mut stream = consumer
        .stream(Offset::end())
        .await
        .expect("Failed to create a stream");

    while let Some(Ok(record)) = stream.next().await {
        let value = record.get_value().to_vec();
        let buffer = value.as_slice();
        let message_type = MessageType::from(buffer[2] as u16);

        match message_type {
            MessageType::ClientError => {
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
                        break;
                    }
                    ClientErrorType::ClientNotLoggedIn => {
                        // The gateway sends this error when the client is not logged in.
                        // This means, the client cannot connect to the gateway and thus must login first.
                        println!("ClientNotLoggedIn");
                        // This is a fatal error, and the client must try to login again probably with a different client id.
                        break;
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
            }
            // Handle data errors
            MessageType::DataError => {
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
                    } //
                }
                // Break out of the loop if we get any DataError b/c these are non-recoverable.
                // In that case, the client should shutdown, restart, and request correct data.
                break;
            }
            // Ignore other message types
            _ => {}
        }
    }

    Ok(())
}

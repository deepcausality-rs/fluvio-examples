use crate::service::Server;
use autometrics::autometrics;
use common::prelude::{ClientChannel, MessageProcessingError};
use fluvio::{Fluvio, RecordKey};
use sbe_messages::prelude::{ClientErrorMessage, ClientErrorType, ClientLogoutMessage};

impl Server {
    /// Handles a client logout message by validating the client ID and logging them out.
    ///
    /// Gets the client's control channel, checks if they are logged in, and logs them out if so.
    /// Sends back any errors over the control channel.
    ///
    /// # Parameters
    ///
    /// - `client_logout_msg`: The incoming ClientLogoutMessage from the client
    ///
    /// # Returns
    ///
    /// Result with no value if successful, or a MessageProcessingError if an error occurs.
    ///
    /// # Errors
    ///
    /// - MessageProcessingError if there is an issue getting the client's control channel, checking their login status,
    /// or logging them out.
    ///
    #[autometrics]
    pub(crate) async fn handle_client_logout(
        &self,
        client_logout_msg: &ClientLogoutMessage,
    ) -> Result<(), MessageProcessingError> {
        // Remove debug print
        println!(
            "[QDGW/handle_client::client_logout]: {:?}",
            client_logout_msg
        );

        // Extract the client ID from the message
        let client_id = client_logout_msg.client_id();

        // Get the client's control channel to return error messages back to the client
        let client_control_channel = match self
            .get_client_channel(ClientChannel::ControlChannel, client_id)
            .await
        {
            Ok(channel) => channel,
            Err(e) => {
                return Err(e);
            }
        };

        // Connect to the Fluvio cluster
        let fluvio = Fluvio::connect().await.unwrap();

        // Get the producer for the client's control channel
        let producer = fluvio
            .topic_producer(client_control_channel)
            .await
            .expect("Failed to create a producer");

        // Check if the client is logged in
        let exists = self.check_client_login(client_id).await;

        match exists {
            Ok(exists) => match exists {
                // client exists, proceed with logout
                true => {
                    let res = self.client_logout(client_id).await;

                    match res {
                        Ok(_) => {}
                        Err(err) => {
                            println!(
                                "[QDGW/handle_client_logout::handle_client_logout] ClientLogOutError: {:?}",
                                err.to_string()
                            );

                            let client_error_type = ClientErrorType::ClientLogOutError;
                            let message = ClientErrorMessage::new(client_id, client_error_type);
                            let enc = message.encode();
                            assert!(enc.is_ok());
                            let (_, buffer) = enc.unwrap();

                            producer
                                .send(RecordKey::NULL, buffer)
                                .await
                                .expect("Failed to send ClientError: ClientLogInError!");
                            producer.flush().await.expect("Failed to flush");
                        }
                    }
                }
                // client does not exist, return an ClientNotLoggedIn error to the client
                false => {
                    let client_error_type = ClientErrorType::ClientNotLoggedIn;
                    let message = ClientErrorMessage::new(client_id, client_error_type);
                    let enc = message.encode();
                    assert!(enc.is_ok());
                    let (_, buffer) = enc.unwrap();

                    producer
                        .send(RecordKey::NULL, buffer)
                        .await
                        .expect("Failed to send ClientError: ClientNotLoggedIn!");
                    producer.flush().await.expect("Failed to flush");
                }
            },
            // Something went horribly wrong, log the message, and return an unknown error
            Err(err) => {
                println!(
                    "[QDGW/handle_client_logout::handle_client_logout] UnknownClientError: {:?}",
                    err.to_string()
                );

                let client_error_type = ClientErrorType::UnknownClientError;
                let message = ClientErrorMessage::new(client_id, client_error_type);
                let enc = message.encode();
                assert!(enc.is_ok());

                let (_, buffer) = enc.unwrap();

                producer
                    .send(RecordKey::NULL, buffer)
                    .await
                    .expect("Failed to send ClientError: UnknownClientError!");
                producer.flush().await.expect("Failed to flush");
            }
        }

        Ok(())
    }

    /// Logs out a client by removing them from the client database.
    ///
    /// Locks the client manager and removes the client with the given ID.
    ///
    /// # Parameters
    ///
    /// - `client_id`: The ID of the client to log out
    ///
    /// # Returns
    ///
    /// A Result with no value if the client was logged out successfully,
    /// or a MessageProcessingError if there was an issue.
    ///
    /// # Errors
    ///
    /// - MessageProcessingError if there was an issue removing the client from the database.
    ///
    pub(crate) async fn client_logout(&self, client_id: u16) -> Result<(), MessageProcessingError> {
        let mut client_db = self.client_manager.lock().await.clone();
        client_db.remove_client(client_id);

        Ok(())
    }
}

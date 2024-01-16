use crate::service::Server;
use common::prelude::{ClientChannel, MessageClientConfig, MessageProcessingError};
use fluvio::Fluvio;
use sbe_messages::prelude::{ClientErrorType, ClientLoginMessage};
use autometrics::autometrics;

impl Server {
    /// Handles a client login message by validating the client ID and logging them in.
    ///
    /// Gets the client's control channel, checks if they are already logged in, and logs them in if not.
    /// Sends back any errors over the control channel.
    ///
    /// # Parameters
    ///
    /// - `client_login_msg`: The incoming ClientLoginMessage from the client
    ///
    /// # Returns
    ///
    /// Result with no value if successful, or a MessageProcessingError if an error occurs.
    ///
    /// # Errors
    ///
    /// - MessageProcessingError if there is an issue getting the client's control channel, checking their login status,
    /// or logging them in.
    ///
    /// ```
    #[autometrics]
    pub(crate) async fn handle_client_login(
        &self,
        client_login_msg: &ClientLoginMessage,
    ) -> Result<(), MessageProcessingError> {
        // Remove debug print
        println!(
            "[QDGW/handle_client::handle_client_login]: {:?}",
            &client_login_msg
        );

        // Extract the client ID from the message
        let client_id = client_login_msg.client_id();

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

        // Check if the client is already logged in
        let exists = self.check_client_login(client_id).await;

        // If the client is already logged in, return an error
        // If not, proceed with client login
        match exists {
            // Client already logged in, return an error back to the client
            Ok(exists) => match exists {
                true => {
                    let client_error_type = ClientErrorType::ClientAlreadyLoggedIn;
                    match self
                        .send_client_error(&producer, client_id, client_error_type)
                        .await
                    {
                        Ok(_) => {}
                        Err(err) => {
                            println!("[QDGW/handle_client_login::handle_client_login] ClientAlreadyLoggedIn: {:?}", err);
                        }
                    }
                }
                // Client not logged in, proceed with login
                false => {
                    let res = self.client_login(client_id).await;

                    match res {
                        Ok(_) => {}
                        Err(err) => {
                            println!("[QDGW/handle_client_login::handle_client_login] ClientLogInError: {:?}", err.to_string());

                            let client_error_type = ClientErrorType::ClientLogInError;
                            match self
                                .send_client_error(&producer, client_id, client_error_type)
                                .await
                            {
                                Ok(_) => {}
                                Err(err) => {
                                    println!("[QDGW/handle_client_login::handle_client_login] ClientLogInError: {:?}", err.to_string());
                                }
                            }
                        }
                    }
                }
            },
            // Something went horribly wrong, log the message, and return an unknown error
            Err(err) => {
                println!(
                    "[QDGW/handle_client_login::handle_client_login] UnknownClientError: {:?}",
                    err.to_string()
                );

                let client_error_type = ClientErrorType::UnknownClientError;
                match self
                    .send_client_error(&producer, client_id, client_error_type)
                    .await
                {
                    Ok(_) => {}
                    Err(err) => {
                        println!("[QDGW/handle_client_login::handle_client_login] UnknownClientError: {:?}", err.to_string());
                    }
                }
            }
        }

        Ok(())
    }

    /// Login a client by adding them to the client database.
    ///
    /// Locks the client manager, creates a config for the client,
    /// and attempts to add them to the database.
    ///
    /// # Parameters
    ///
    /// - `client_id`: The ID of the client to log in
    ///
    /// # Returns
    ///
    /// A Result with no value if the client was logged in successfully,
    /// or a MessageProcessingError if there was an issue.
    ///
    /// # Errors
    ///
    /// - MessageProcessingError if there was an issue adding the client to the database.
    ///
    /// ```
    pub(crate) async fn client_login(&self, client_id: u16) -> Result<(), MessageProcessingError> {
        let mut client_db = self.client_manager.lock().await.clone();
        let config = MessageClientConfig::new(client_id);

        match client_db.add_client(client_id, config) {
            Ok(_) => Ok(()),
            Err(e) => Err(MessageProcessingError(e.to_string())),
        }
    }
}

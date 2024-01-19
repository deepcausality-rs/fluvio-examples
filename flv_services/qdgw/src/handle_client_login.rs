use crate::service::Server;
use autometrics::autometrics;
use common::prelude::{ClientChannel, MessageClientConfig, MessageProcessingError};
use sbe_messages::prelude::{ClientErrorType, ClientLoginMessage};

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
        // println!("[QDGW/handle_client::handle_client_login]:");

        // println!("::handle_client_login]: Extract the client ID from the message");
        let client_id = client_login_msg.client_id();

        // println!("::handle_client_login]: Check if the client is already logged in");
        let exists = self.check_client_login(client_id).await;

        // If the client is already logged in, return an error
        // If not, proceed with client login
        match exists {
            Ok(exists) => match exists {
                true => {
                    // println!("::handle_client_login]: Client already logged in, return an error back to the client");
                    let client_error_type = ClientErrorType::ClientAlreadyLoggedIn;
                    match self.send_client_error(client_id, client_error_type).await {
                        Ok(_) => {}
                        Err(err) => {
                            println!("[QDGW/handle_client_login::handle_client_login] ClientAlreadyLoggedIn: {:?}", err);
                        }
                    }
                }
                //
                false => {
                    // println!("::handle_client_login]: Client not logged in, proceed with login");
                    let res = self.client_login(client_id).await;

                    match res {
                        Ok(_) => {}
                        Err(err) => {
                            println!("[QDGW/handle_client_login::handle_client_login] ClientLogInError: {:?}", err.to_string());

                            let client_error_type = ClientErrorType::ClientLogInError;
                            match self.send_client_error(client_id, client_error_type).await {
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
                    err
                );

                let client_error_type = ClientErrorType::UnknownClientError;
                match self.send_client_error(client_id, client_error_type).await {
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
        // Lock the client_manager
        let mut client_db = self.client_manager.lock().await;

        // Create a new config for the client
        let config = MessageClientConfig::new(client_id);

        // Add the client to the client_manager
        match client_db.add_client(client_id, config) {
            Ok(_) => {
                // println!("[::client_login]: Client logged in successfully");
            }
            Err(e) => {
                println!("[::client_login]: Failed to add client to client_manager");
                return Err(MessageProcessingError(e.to_string()));
            }
        }

        // Unlock the client_manager
        drop(client_db);

        // create a new client topic producer for the client
        let producer = self
            .get_channel_producer(ClientChannel::DataChannel, client_id)
            .await
            .expect("[send_error]: Failed to get error channel producer");

        // lock the client_data_producers hashmap
        let mut client_data_producers = self.client_data_producers.lock().await;

        // add the client data producer to the hashmap
        client_data_producers.insert(client_id, producer);

        // Unlock the client_data_producers hashmap
        drop(client_data_producers);

        println!(
            "[client_login]: Client {:?} logged in successfully",
            client_id
        );

        Ok(())
    }
}

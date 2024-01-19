use crate::service::Server;
use autometrics::autometrics;
use common::prelude::{ClientChannel, MessageProcessingError};
use fluvio::Fluvio;
use sbe_messages::prelude::{ClientErrorType, ClientLogoutMessage};

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

        println!("::handle_client_logout]: Extract the client ID from the message");
        let client_id = client_logout_msg.client_id();

        println!("::handle_client_logout]: Get the client's control channel to send messages back to the client");
        let client_control_channel = match self
            .get_client_channel(ClientChannel::ControlChannel, client_id)
            .await
        {
            Ok(channel) => channel,
            Err(e) => {
                return Err(e);
            }
        };

        println!("::handle_client_logout]: Connect to the Fluvio cluster");
        let fluvio = Fluvio::connect().await.unwrap();

        println!("::handle_client_logout]: Get the producer for the client's control channel");
        let producer = fluvio
            .topic_producer(client_control_channel)
            .await
            .expect("Failed to create a producer");

        println!("::handle_client_logout]: Check if the client is logged in");
        let exists = self.check_client_login(client_id).await;

        match exists {
            Ok(exists) => match exists {
                true => {
                    println!("[::handle_client_logout]: Client is logged in, proceed with logout");

                    let res = self.client_logout(client_id).await;
                    match res {
                        Ok(_) => {}
                        Err(err) => {
                            println!("[QDGW/handle_client_logout::handle_client_logout] ClientLogOutError: {:?}", err);

                            let client_error_type = ClientErrorType::ClientLogOutError;
                            match self
                                .send_client_error(client_id, client_error_type)
                                .await {
                                Ok(_) => {}
                                Err(err) => println!("[QDGW/handle_client_login::handle_client_login] ClientLogInError: {:?}", err.to_string()),
                            }
                        }
                    }
                }
                // client does not exist, return an ClientNotLoggedIn error to the client
                false => {
                    println!("[::handle_client_logout]: Client is not logged in, return an ClientNotLoggedIn error to the client");

                    let client_error_type = ClientErrorType::ClientNotLoggedIn;
                    match self.send_client_error(client_id, client_error_type).await {
                        Ok(_) => {}
                        Err(err) => {
                            println!("[QDGW/handle_client_login::handle_client_login] ClientAlreadyLoggedIn: {:?}", err);
                        }
                    }
                }
            },
            // Something went horribly wrong, log the message, and return an unknown error
            Err(err) => {
                println!(
                    "[QDGW/handle_client_logout::handle_client_logout] UnknownClientError: {:?}",
                    err.to_string()
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
        let mut client_db = self.client_manager.lock().await;
        client_db.remove_client(client_id);
        drop(client_db);

        println!("[::client_logout]: Client logged out successfully");

        Ok(())
    }
}

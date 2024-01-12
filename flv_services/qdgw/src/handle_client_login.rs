use crate::service::Server;
use common::prelude::{ClientChannel, MessageClientConfig, MessageProcessingError};
use fluvio::{Fluvio, RecordKey};
use sbe_messages::prelude::{ClientErrorMessage, ClientErrorType, ClientLoginMessage};

impl Server {
    pub(crate) async fn handle_client_login(
        &self,
        client_login_msg: &ClientLoginMessage,
    ) -> Result<(), MessageProcessingError> {
        // Remove debug print
        println!("[QDGW/handle_client::client_login]: {:?}", client_login_msg);

        let client_id = client_login_msg.client_id();

        let client_control_channel = match self
            .get_client_channel(ClientChannel::ControlChannel, client_id)
            .await
        {
            Ok(channel) => channel,
            Err(e) => {
                return Err(e);
            }
        };

        let fluvio = Fluvio::connect().await.unwrap();

        let producer = fluvio
            .topic_producer(client_control_channel)
            .await
            .expect("Failed to create a producer");

        let exists = self.check_client_login(client_id).await;

        match exists {
            // Client already logged in, return an error
            Ok(exists) => match exists {
                true => {
                    let client_error_type = ClientErrorType::ClientAlreadyLoggedIn;
                    let message = ClientErrorMessage::new(client_id, client_error_type);
                    let enc = message.encode();
                    assert!(enc.is_ok());
                    let (_, buffer) = enc.unwrap();

                    producer
                        .send(RecordKey::NULL, buffer)
                        .await
                        .expect("Failed to send ClientError: ClientAlreadyLoggedIn!");
                    producer.flush().await.expect("Failed to flush");
                }
                // Client not logged in, proceed with login
                false => {
                    let res = self.client_login(client_id).await;

                    match res {
                        Ok(_) => {}
                        Err(err) => {
                            println!(
                                "[QDGW/handle_client_login::handle_client_login] ClientLogInError: {:?}",
                                err.to_string()
                            );

                            let client_error_type = ClientErrorType::ClientLogInError;
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
            },
            // Something went horribly wrong, log, and return an unknown error
            Err(err) => {
                println!(
                    "[QDGW/handle_client_login::handle_client_login] UnknownClientError: {:?}",
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

    pub(crate) async fn client_login(&self, client_id: u16) -> Result<(), MessageProcessingError> {
        let mut client_db = self.client_manager.lock().await.clone();
        let config = MessageClientConfig::new(client_id);

        return match client_db.add_client(client_id, config) {
            Ok(_) => Ok(()),
            Err(e) => Err(MessageProcessingError(e.to_string())),
        };
    }
}

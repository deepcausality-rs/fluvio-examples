use iggy::client::MessageClient;
use iggy::messages::send_messages::{Message, Partitioning, SendMessages};
use warp::hyper::body::Bytes;

use common::prelude::MessageProcessingError;
use sbe_messages::prelude::{ClientErrorMessage, ClientErrorType, DataErrorMessage, DataErrorType};

use crate::service::Server;

impl Server {
    /// Sends a ClientError message to the given producer.
    ///
    /// # Parameters
    ///
    /// * `producer` - The topic producer to send the message on
    /// * `client_id` - The id of the client the error is for
    /// * `client_error` - The ClientErrorType to send
    ///
    /// # Returns
    ///
    /// Returns a `Result` with `()` if successful, otherwise returns a
    /// `MessageProcessingError` on failure to send.
    ///
    pub(crate) async fn send_client_error(
        &self,
        client_id: u16,
        client_error: ClientErrorType,
    ) -> Result<(), MessageProcessingError> {
        let message = ClientErrorMessage::new(client_id, client_error);
        let enc = message.encode();
        assert!(enc.is_ok());

        let (_, buffer) = enc.unwrap();

        // Build iggy message wrapper
        let message = Message::new(None, Bytes::from(buffer), None);

        self.send_error(message)
            .await
            .expect("Failed to send client error message");

        Ok(())
    }

    /// Sends a DataError message to the given producer.
    ///
    /// # Parameters
    ///
    /// * `producer` - The topic producer to send the message on
    /// * `client_id` - The id of the client the error is for
    /// * `data_error` - The DataErrorType to send
    ///
    /// # Returns
    ///
    /// Returns a `Result` with `()` if successful, otherwise returns a
    /// `MessageProcessingError` on failure to send.
    ///
    pub(crate) async fn send_data_error(
        &self,
        client_id: u16,
        data_error: DataErrorType,
    ) -> Result<(), MessageProcessingError> {
        let message = DataErrorMessage::new(client_id, data_error);
        let (_, buffer) = message
            .encode()
            .expect("Failed to encode data error message");

        // Build iggy message wrapper
        let message = Message::new(None, Bytes::from(buffer), None);

        // Send message
        self.send_error(message)
            .await
            .expect("Failed to send error message");

        Ok(())
    }

    /// Sends an error message to the given producer.
    ///
    /// # Parameters
    ///
    /// * `producer` - The topic producer to send the message on
    /// * `error_buffer` - The encoded error message bytes to send
    ///
    /// # Returns
    ///
    /// Returns a `Result` with `()` if successful, otherwise returns a
    /// `MessageProcessingError` on failure to send.
    ///
    pub(crate) async fn send_error(&self, message: Message) -> Result<(), MessageProcessingError> {
        self.producer()
            .send_messages(&mut SendMessages {
                stream_id: self.iggy_config().stream_id(),
                topic_id: self.iggy_config().topic_id(),
                partitioning: Partitioning::partition_id(self.iggy_config().partition_id()),
                messages: vec![message],
            })
            .await
            .expect("Failed to send error");

        // Send the error message

        Ok(())
    }
}

use crate::service::Server;
use common::prelude::{ClientChannel, MessageProcessingError};
use fluvio::RecordKey;
use sbe_messages::prelude::{ClientErrorMessage, ClientErrorType, DataErrorMessage, DataErrorType};

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
        self.send_error(client_id, buffer).await?;

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
        let enc = message.encode();
        assert!(enc.is_ok());

        let (_, buffer) = enc.unwrap();
        self.send_error(client_id, buffer).await?;

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
    pub(crate) async fn send_error(
        &self,
        client_id: u16,
        error_buffer: Vec<u8>,
    ) -> Result<(), MessageProcessingError> {
        // Get the producer for the error channel
        let producer = self
            .get_channel_producer(ClientChannel::ErrorChannel, client_id)
            .await
            .expect("[send_error]: Failed to get error channel producer");

        // Send the error message
        producer
            .send(RecordKey::NULL, error_buffer)
            .await
            .expect("[send_error]: Failed to send DataError: DataUnavailableError!");

        // Flush the producer to the message bus
        producer
            .flush()
            .await
            .expect("[send_error]: Failed to flush to message bus.");

        Ok(())
    }
}

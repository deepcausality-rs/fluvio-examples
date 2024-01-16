use crate::service::Server;
use common::prelude::MessageProcessingError;
use fluvio::dataplane::record::ConsumerRecord;
use sbe_messages::prelude::{
    ClientLoginMessage, ClientLogoutMessage, MessageType, StartDataMessage, StopAllDataMessage,
    StopDataMessage,
};

impl Server {
    /// Handles a consumer record received from the Fluvio message bus.
    ///
    /// Parses the message type and delegates to the appropriate handler.
    ///
    /// # Parameters
    ///
    /// * `record` - The Fluvio consumer record to handle
    ///
    /// # Returns
    ///
    /// Returns a `Result` with `()` if successful, otherwise returns a
    /// `MessageProcessingError` on failure.
    ///
    /// # Errors
    ///
    /// Can fail with a `MessageProcessingError` if:
    ///
    /// - An unknown message type is received
    /// - Any of the delegated handlers fail
    ///
    pub(crate) async fn handle_record(
        &self,
        record: &ConsumerRecord,
    ) -> Result<(), MessageProcessingError> {
        //
        let value = record.get_value().to_vec();
        let buffer = value.as_slice();
        let message_type = MessageType::from(buffer[2] as u16);

        match message_type {
            MessageType::ClientLogin => {
                let client_login_msg = ClientLoginMessage::from(buffer);
                self.handle_client_login(&client_login_msg).await
            }

            MessageType::ClientLogout => {
                let client_logout_msg = ClientLogoutMessage::from(buffer);
                self.handle_client_logout(&client_logout_msg).await
            }

            MessageType::StartData => {
                let start_data_msg = StartDataMessage::from(buffer);
                self.handle_start_data_message(&start_data_msg).await
            }

            MessageType::StopData => {
                let stop_data_msg = StopDataMessage::from(buffer);
                self.handle_stop_date(&stop_data_msg).await
            }
            MessageType::StopAllData => {
                let stop_all_data_msg = StopAllDataMessage::from(buffer);
                self.handle_stop_all_data(&stop_all_data_msg).await
            }
            _ => {
                Err(MessageProcessingError(
                    "[QDGW/handle::handle_record]:  Fluvio consumer record contained an unknown message type."
                        .to_string(),
                ))
            }
        }
    }
}

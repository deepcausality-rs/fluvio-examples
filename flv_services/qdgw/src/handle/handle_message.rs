use crate::service::Server;
use common::prelude::MessageProcessingError;
use sbe_messages::prelude::{
    ClientLoginMessage, ClientLogoutMessage, MessageType, StartDataMessage, StopAllDataMessage,
    StopDataMessage,
};

impl Server {
    /// Handles a single message by processing it and sending it to the appropriate
    /// manager for further processing.
    ///
    /// This method takes a message payload, processes it by calling the `process_message`
    /// method, and sends it to the appropriate handler for further processing.
    ///
    /// # Parameters
    ///
    /// * `self` - The Server instance
    /// * `message` - The message payload to be processed
    ///
    /// # Returns
    /// * Ok on success,
    /// * Err on any processing error
    ///
    pub(crate) async fn handle_message(
        &self,
        raw_message: &[u8],
    ) -> Result<(), MessageProcessingError> {
        //
        let message_type = MessageType::from(raw_message[2] as u16);

        match message_type {
            MessageType::ClientLogin => {
                let client_login_msg = ClientLoginMessage::from(raw_message);
                self.handle_client_login(&client_login_msg).await
            }

            MessageType::ClientLogout => {
                let client_logout_msg = ClientLogoutMessage::from(raw_message);
                self.handle_client_logout(&client_logout_msg).await
            }

            MessageType::StartData => {
                let start_data_msg = StartDataMessage::from(raw_message);
                self.handle_start_data_message(&start_data_msg).await
            }

            MessageType::StopData => {
                let stop_data_msg = StopDataMessage::from(raw_message);
                self.handle_stop_date(&stop_data_msg).await
            }
            MessageType::StopAllData => {
                let stop_all_data_msg = StopAllDataMessage::from(raw_message);
                self.handle_stop_all_data(&stop_all_data_msg).await
            }
            _ => Err(MessageProcessingError(
                "[QDGW/handle::handle_record]: Unknown message type. Abort processing".to_string(),
            )),
        }
    }
}

use std::error::Error;

use bytes::Bytes;
use iggy::messages::send_messages::Message;

use sbe_messages::prelude::ClientLogoutMessage;

use crate::QDClient;

impl QDClient {
    /// Logs out of the client by sending a logout message to the gateway.
    ///
    /// # Returns
    ///
    /// Returns a `Result` with `()` on success, or an `Error` on failure.
    ///
    /// This constructs a `ClientLogoutMessage` with the client ID.
    /// It encodes the message to bytes and sends it to the gateway
    /// using the `send_message` method.
    ///
    pub(crate) async fn logout(&self) -> Result<(), Box<dyn Error + Send>> {
        // Construct logout message
        let logout_message = ClientLogoutMessage::new(self.client_id);

        // Encode message
        let (_, buffer) = logout_message
            .encode()
            .expect("[QDClient/logout]: Failed to encode message");

        // Build iggy message wrapper
        let message = Message::new(None, Bytes::from(buffer), None);

        // Send message to the gateway
        self.send_message(message)
            .await
            .expect("[QDClient/logout]: Failed to send logout message!");

        Ok(())
    }
}

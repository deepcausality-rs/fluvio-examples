use crate::QDClient;
use sbe_messages::prelude::ClientLoginMessage;
use std::error::Error;

impl QDClient {
    /// Logs in the client by sending a login message to the gateway.
    ///
    /// # Returns
    ///
    /// Returns a `Result` with `()` on success, or an `Error` on failure.
    ///
    /// This constructs a `ClientLoginMessage` with the client ID.
    /// It encodes the message to bytes and sends it to the gateway
    /// using the `send_message` method.
    ///
    pub(crate) async fn login(&self) -> Result<(), Box<dyn Error + Send>> {
        // Construct login message
        let message = ClientLoginMessage::new(self.client_id);

        // Encode message
        let (_, buffer) = message
            .encode()
            .expect("[QDClient/login]: Failed to encode message");

        // Send message to the gateway
        self.send_message(buffer)
            .await
            .expect("[QDClient/login]: Failed to send login message!");

        Ok(())
    }
}

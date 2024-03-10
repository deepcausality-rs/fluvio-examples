use common::prelude::MessageProcessingError;

use crate::service::Server;

impl Server {
    /// Checks if a client with the given ID is logged in.
    ///
    /// Locks the client manager mutex and checks if the client ID exists.
    ///
    /// # Parameters
    ///
    /// - `client_id`: The ID of the client to check
    ///
    /// # Returns
    ///
    /// A `Result` with a `bool` indicating whether the client is logged in, or a
    /// `MessageProcessingError` if there was an issue checking the client status.
    ///
    pub(crate) async fn check_client_login(
        &self,
        client_id: u16,
    ) -> Result<bool, MessageProcessingError> {
        let client_db = self.client_producers().read().await;

        Ok(client_db.contains_key(&client_id))
    }
}

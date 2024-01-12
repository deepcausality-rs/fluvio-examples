use crate::service::Server;
use common::prelude::MessageProcessingError;
use sbe_messages::prelude::ClientLogoutMessage;

impl Server {
    pub(crate) async fn client_logout(
        &self,
        client_logout_msg: &ClientLogoutMessage,
    ) -> Result<(), MessageProcessingError> {
        // Remove debug print
        println!(
            "[QDGW/handle_client::client_logout]: {:?}",
            client_logout_msg
        );

        let mut client_db = self.client_manager.lock().await.clone();

        let exists = client_db.check_client(client_logout_msg.client_id());

        // Return a proper error message to the client if the client already exists
        if !exists {
            return Err(MessageProcessingError(
                "[QDGW/handle_client::client_login]: Client does not exists".to_string(),
            ));
        }

        client_db.remove_client(client_logout_msg.client_id());

        Ok(())
    }
}

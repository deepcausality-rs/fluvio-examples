use crate::service::Server;
use client_manager::ClientManager;
use common::prelude::{ClientChannel, MessageProcessingError};
use std::sync::{Arc, Mutex};

impl Server {
    /// Returns the channel name for the given client and channel type.
    ///
    /// Locks the ClientManager mutex and looks up the appropriate channel based on
    /// the ClientChannel enum.
    ///
    /// # Parameters
    ///
    /// - `client_manager` - The ClientManager instance
    /// - `client_channel` - The ClientChannel enum specifying the channel type
    /// - `client_id` - The id of the client
    ///
    /// # Returns
    ///
    /// The name of the channel as a String, or a MessageProcessingError if the lookup fails.
    ///
    pub(crate) async fn get_client_channel(
        &self,
        client_manager: &Arc<Mutex<ClientManager>>,
        client_channel: ClientChannel,
        client_id: u16,
    ) -> Result<String, MessageProcessingError> {
        // Lock the ClientManager mutex
        let client_db = client_manager.lock().unwrap();

        // Look up the channel
        let res = match client_channel {
            ClientChannel::DataChannel => client_db.get_client_data_channel(client_id),
            ClientChannel::ControlChannel => client_db.get_client_control_channel(client_id),
            ClientChannel::ExecutionChannel => client_db.get_client_execution_channel(client_id),
            ClientChannel::HeartbeatChannel => client_db.get_client_heartbeat_channel(client_id),
        };

        // Return the channel, or an error if the lookup failed
        match res {
            Ok(channel) => Ok(channel),
            Err(e) => Err(MessageProcessingError(e.to_string())),
        }
    }
}

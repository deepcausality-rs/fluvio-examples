use crate::service::Server;
use common::prelude::{ClientChannel, MessageProcessingError};


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
        client_channel: ClientChannel,
        client_id: u16,
    ) -> Result<String, MessageProcessingError> {
        // Lock the ClientManager
        let client_db = self.client_manager.lock().await.clone();

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

    /// Retrieves the trade table name for the given exchange ID.
    ///
    /// Locks the SymbolManager mutex and looks up the table name for the exchange.
    ///
    /// # Parameters
    ///
    /// - `symbol_manager` - The SymbolManager instance
    /// - `exchange_id` - The exchange ID
    ///
    /// # Returns
    ///
    /// The name of the trade table as a `String`, or a `MessageProcessingError` if lookup fails.
    ///
    pub(crate) async fn get_trade_table_name(
        &self,
        exchange_id: u8,
    ) -> Result<String, MessageProcessingError> {
        // Lock the SymbolManager
        let mut symbol_db = self.symbol_manager.lock().await;

        // look up the table name
        let res = symbol_db.get_symbol_table(exchange_id as u16);

        // Return the table name, or an error if the lookup failed
        match res {
            Ok(table_name) => Ok(table_name),
            Err(e) => Err(MessageProcessingError(e.to_string())),
        }
    }
}

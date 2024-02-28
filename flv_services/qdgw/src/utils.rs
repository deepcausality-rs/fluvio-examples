use crate::service::Server;
use common::prelude::MessageProcessingError;

impl Server {
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
        exchange_id: u16,
        symbol_id: u16,
    ) -> Result<String, MessageProcessingError> {
        // Lock the SymbolManager
        let mut symbol_db = self.symbol_manager.write().await;

        // look up the table name
        let res = symbol_db.get_symbol_table_name(exchange_id, symbol_id);

        // Return the table name, or an error if the lookup failed
        match res {
            Ok(table_name) => Ok(table_name),
            Err(e) => Err(MessageProcessingError(e.to_string())),
        }
    }

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
        let client_db = self.client_manager.read().await;

        let exists = client_db.check_client(client_id);

        Ok(exists)
    }
}

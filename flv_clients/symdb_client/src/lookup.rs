use crate::error::SymdbClientError;
use crate::{utils_proto, SymdbClient};
use common::prelude::ExchangeID;

const FN_NAME: &str = "[SymdbClient]: ";

impl SymdbClient {
    /// Looks up the exchange name for a given exchange ID.
    ///
    /// # Arguments
    ///
    /// * `exchange_id` - The u32 ID of the exchange to look up
    ///
    /// # Returns
    ///
    /// Returns a Result with the exchange name string if found, otherwise a SymdbClientError.
    ///
    /// # Example
    ///
    ///
    /// let exchange_name = lookup_exchange_name(1).await?;
    ///
    pub async fn lookup_exchange_name(
        &mut self,
        exchange_id: ExchangeID,
    ) -> Result<String, SymdbClientError> {
        //
        let request = utils_proto::get_exchange_request(exchange_id);

        let res = self.client.lookup_exchange_name(request).await;

        match res {
            Ok(res) => Ok(res.into_inner().exchange_name),
            Err(err) => Err(SymdbClientError(err.to_string())),
        }
    }

    /// Looks up the symbol string for a given symbol ID.
    ///
    /// # Arguments
    ///
    /// * `symbol_id` - The u32 ID of the symbol to look up
    ///
    /// # Returns
    ///
    /// Returns a Result with the symbol string if found, otherwise a SymdbClientError.
    ///
    ///
    /// # Example
    ///
    /// let symbol = lookup_symbol(42).await?;
    ///
    pub async fn lookup_symbol(
        &mut self,
        exchange_id: ExchangeID,
        symbol_id: u16,
    ) -> Result<String, SymdbClientError> {
        let request = utils_proto::get_symbol_request(exchange_id, symbol_id);

        let res = self.client.lookup_symbol(request).await;

        match res {
            Ok(res) => Ok(res.into_inner().symbol),
            Err(err) => Err(get_error(
                format!("Error Looking up symbol for ID {}", symbol_id).as_str(),
                &err.to_string(),
            )),
        }
    }

    /// Looks up the symbol ID for a given symbol string.
    ///
    /// # Arguments
    ///
    /// * `symbol` - The symbol string to look up
    ///
    /// # Returns
    ///
    /// Returns a Result with the u32 symbol ID if found, otherwise a SymdbClientError.
    ///
    pub async fn lookup_symbol_id(
        &mut self,
        exchange_id: ExchangeID,
        symbol: String,
    ) -> Result<u16, SymdbClientError> {
        let request = utils_proto::get_symbol_id_request(exchange_id, symbol.clone());

        let res = self.client.lookup_symbol_id(request);

        match res.await {
            Ok(res) => Ok(res.into_inner().symbol_id as u16),
            Err(err) => Err(get_error(
                format!("Error Looking up ID for symbol {}", &symbol).as_str(),
                &err.to_string(),
            )),
        }
    }
}

/// Creates a SymdbClientError with a formatted error message.
///
/// # Arguments
///
/// * `msg` - The message describing what failed.
/// * `err` - The underlying error message.
///
/// # Returns
///
/// Returns a SymdbClientError struct containing the formatted error message.
///
fn get_error(msg: &str, err: &str) -> SymdbClientError {
    SymdbClientError(format!("{} {} because of Error {}", FN_NAME, msg, err))
}

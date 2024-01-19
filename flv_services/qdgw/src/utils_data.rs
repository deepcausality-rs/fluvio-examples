use crate::service::Server;
use common::prelude::{OHLCVBar, TimeResolution, TradeBar};
use db_query_manager::error::QueryError;

impl Server {
    /// Gets all trade bars for the given symbol id and trade table.
    ///
    /// # Parameters
    ///
    /// * `symbol_id` - The numeric id of the symbol to get bars for.
    /// * `trade_table` - The name of the trade table to query.
    ///
    /// # Returns
    ///
    /// Returns a `Result` with a `Vec` of `TradeBar` structs if successful,
    /// otherwise returns a `QueryError` on failure.
    ///
    pub(crate) async fn get_trade_bars(
        &self,
        symbol_id: u16,
        trade_table: &str,
    ) -> Result<Vec<TradeBar>, QueryError> {
        // Lock query manager
        let mut q_manager = self.query_manager.lock().await;

        // Get all bars
        let result = q_manager.get_all_trades(symbol_id, trade_table).await;

        match result {
            Ok(bars) => Ok(bars),
            Err(e) => Err(e),
        }
    }

    /// Gets OHLCV bars for the given symbol id, time resolution and trade table.
    ///
    /// # Parameters
    ///
    /// * `symbol_id` - The numeric id of the symbol to get bars for.
    /// * `time_resolution` - The time resolution to use for the bars.
    /// * `trade_table` - The name of the trade table to query.
    ///
    /// # Returns
    ///
    /// Returns a `Result` with a `Vec` of `OHLCVBar` structs if successful,
    /// otherwise returns a `QueryError` on failure.
    ///
    pub(crate) async fn get_ohlcv_bars(
        &self,
        symbol_id: u16,
        time_resolution: &TimeResolution,
        trade_table: &str,
    ) -> Result<Vec<OHLCVBar>, QueryError> {
        // Lock query manager
        let mut q_manager = self.query_manager.lock().await;

        // Get all bars
        let result = q_manager
            .get_all_ohlcv_bars(symbol_id, trade_table, time_resolution)
            .await;

        // Unlock / drop query manager
        drop(q_manager);

        match result {
            Ok(bars) => Ok(bars),
            Err(e) => Err(e),
        }
    }
}

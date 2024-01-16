use common::prelude::TradeBar;
use db_query_manager::error::QueryError;
use crate::service::Server;

impl Server {
    pub(crate) async fn get_trade_bars(
        &self,
        symbol_id: u16,
        trade_table: &str,
    ) -> Result<Vec<TradeBar>, QueryError> {
        // Lock query manager
        let mut q_manager = self.query_manager.lock().await;

        // Get all bars
        let result = q_manager.get_all_trades(symbol_id, trade_table).await;

        // Unlock / drop query manager
        drop(q_manager);

        match result {
            Ok(bars) => Ok(bars),
            Err(e) => Err(e),
        }
    }
}
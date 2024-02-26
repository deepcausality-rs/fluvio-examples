use crate::types::TradeRow;
use crate::{QueryDBManager, FN_NAME};
use futures::stream::BoxStream;
use futures::StreamExt;
use klickhouse::KlickhouseError;

impl QueryDBManager {
    /// Stream trade bars for the given symbol from the database.
    ///
    /// This returns a stream of `TradeBar` structs for the specified `symbol_id`.
    /// Trade bars are fetched from the database and yielded as they become available.
    ///
    /// # Arguments
    ///
    /// * `symbol_id` - The symbol ID to fetch trade bars for
    /// * `trade_table` - The name of the DB table to query and stream
    ///
    /// # Errors
    ///
    /// This function may return connection errors or other database errors.
    ///
    /// # Example
    ///
    /// ```
    /// use common::prelude::ClickHouseConfig;
    /// use db_query_manager::QueryDBManager;
    ///  use futures::StreamExt;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///
    /// let db_config =  ClickHouseConfig::default();
    ///
    /// let query_manager = QueryDBManager::new(db_config).await.expect("Failed to create db connection");    ///
    /// let trade_table = "kraken_ethaed";
    /// let symbol_id = 284; // 284 = ethaed on Kraken
    ///
    ///     let mut stream = query_manager.stream_trades( trade_table).await;
    ///
    ///     while let Some(record) = stream.next().await {
    ///         assert!(record.is_ok());
    ///         let trade_bar = record.unwrap();
    ///         println!("{:?}", trade_bar);
    ///     }
    /// }
    /// ```
    pub async fn stream_trades<'a>(
        &'a self,
        trade_table: &'a str,
    ) -> BoxStream<Result<TradeRow, KlickhouseError>> {
        let sanitized_name = self
            .sanitize_table_name(trade_table)
            .expect("Invalid table name");

        // Build the query
        let query = self.build_get_trades_query(sanitized_name);

        self.client
            .query::<TradeRow>(query)
            .await
            .expect(format!("{} Failed to execute stream_trades query ", FN_NAME).as_str())
            .boxed()
    }
}

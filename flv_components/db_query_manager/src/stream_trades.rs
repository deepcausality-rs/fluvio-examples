use crate::QueryDBManager;
use common::prelude::TradeBar;
use futures::stream::BoxStream;
use futures::{StreamExt, TryStreamExt};
use sqlx::{Error};

impl QueryDBManager {
    /// Stream trade bars for the given symbol from the database.
    ///
    /// This returns a stream of `TradeBar` structs for the specified `symbol_id`.
    /// Trade bars are fetched in batches from the database and yielded as they
    /// become available.
    ///
    /// # Arguments
    ///
    /// * `symbol_id` - The symbol ID to fetch trade bars for
    /// * `trade_table` - The name of the trade table to query
    ///
    /// # Errors
    ///
    /// This function may return connection errors or other database errors.
    ///
    /// # Example
    ///
    /// ```
    /// use common::prelude::DBConfig;
    /// use db_query_manager::QueryDBManager;
    ///  use futures::StreamExt;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///
    /// let db_config =  DBConfig::new(9009, "0.0.0.0".into());
    ///
    /// let query_manager = QueryDBManager::new(db_config).await.expect("Failed to create db connection");    ///
    /// let trade_table = "kraken_ethaed";    ///
    /// let symbol_id = 284; // 284 = ethaed on Kraken
    ///
    ///     let mut stream = query_manager.stream_trades(symbol_id, trade_table).await;
    ///
    ///     while let  Some(record) = stream.next().await {
    ///         assert!(record.is_ok());
    ///         let record = record.unwrap();
    ///         println!("Got {:?}", record);
    ///     }
    ///
    ///   // Close the connection pool
    ///   query_manager.close().await;
    /// }
    /// ```
    pub async fn stream_trades<'a>(
        &'a self,
        symbol_id: u16,
        trade_table: &'a str,
    ) -> BoxStream<Result<TradeBar, Error>> {

        // returns BoxStream<Result<PgRow, Error>>
        sqlx::query(trade_table)
            .fetch(&self.pool)
            .map_ok(move |row| {
                TradeBar::from_pg_row(symbol_id, row)

            })
            .boxed()
    }
}

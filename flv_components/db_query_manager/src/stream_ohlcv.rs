use crate::QueryDBManager;
use common::prelude::OHLCVBar;
use futures::stream::BoxStream;
use futures::{StreamExt, TryStreamExt};
use sqlx::Error;

impl QueryDBManager {
    pub async fn stream_ohlcv<'a>(
        &'a self,
        symbol_id: u16,
        query: &'a str,
    ) -> BoxStream<Result<OHLCVBar, Error>> {
        // returns BoxStream<Result<PgRow, Error>>
        sqlx::query(query)
            .fetch(&self.pool)
            .map_ok(move |row| OHLCVBar::from_pg_row(row, symbol_id))
            .boxed()
    }
}

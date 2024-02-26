use futures::stream::BoxStream;
use futures::StreamExt;
use klickhouse::KlickhouseError;
use crate::{FN_NAME, QueryDBManager};
use common::prelude::{TimeResolution};
use crate::types::{OHLCVRow};

impl QueryDBManager {
    pub async fn stream_ohlcv<'a>(
        &'a self,
        symbol_table: &str,
        time_resolution: &TimeResolution,
    ) -> BoxStream<Result<OHLCVRow, KlickhouseError>> {

        // Sanitize table name input to prevent SQL injection.
        let sanitized_name = self
            .sanitize_table_name(symbol_table)
            .expect("Failed to sanitize table name");

        // Build the query
        let query = self.build_get_ohlcv_bars_query(sanitized_name, time_resolution);

        // Return the stream of rows
        self
            .client
            .query::<OHLCVRow>(query)
            .await
            .expect(format!("{} Failed to execute stream_ohlcv query ", FN_NAME).as_str())
            .boxed()
    }
}

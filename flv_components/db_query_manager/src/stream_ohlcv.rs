use clickhouse::query::RowCursor;
use crate::{FN_NAME, QueryDBManager};
use common::prelude::{TimeResolution};
use crate::types::{OHLCVRow};

impl QueryDBManager {
    pub async fn stream_ohlcv<'a>(
        &'a self,
        symbol_table: &str,
        time_resolution: &TimeResolution,
    ) -> RowCursor<OHLCVRow> {

        // Sanitize table name input to prevent SQL injection.
        let sanitized_name = self.sanitize_table_name(symbol_table).expect("Failed to sanitize table name");

        // Build the query
        let query = self.build_get_ohlcv_bars_query(sanitized_name, time_resolution);

        // Return the stream of rows
        self
            .client
            .query(&query)
            .fetch::<OHLCVRow>()
            .expect(format!("{} Failed to execute stream query: {}", FN_NAME, query).as_str())
    }
}

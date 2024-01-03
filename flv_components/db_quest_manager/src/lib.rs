use common::prelude::{DBConfig, TradeBar};
use questdb::ingress::Sender;
use questdb::{
    ingress::{Buffer, SenderBuilder, TimestampNanos},
    Result as QuestDBResult,
};
use rust_decimal::prelude::ToPrimitive;

pub struct QuestDBManager {
    db_config: DBConfig,
    sender: Sender,
}

impl QuestDBManager {

    pub fn new(db_config: DBConfig) -> Self {
        let host = db_config.host();
        let port = db_config.port();

        let sender = SenderBuilder::new(host, port)
            .connect()
            .expect("Failed to connect to QuestDB");

        Self { db_config, sender }
    }
}

impl QuestDBManager {

    /// Inserts a batch of trade bars into the specified QuestDB table.
    ///
    /// # Arguments
    ///
    /// * `trade_bars` - The vector of TradeBar items to insert
    /// * `table_name` - The name of the QuestDB table to insert into
    /// * `symbol` - The symbol associated with the trade bars
    ///
    /// # Returns
    ///
    /// Returns a `QuestDBResult` indicating success or failure.
    ///
    /// # Details
    ///
    /// This method batches the trade bars into a buffer before inserting
    /// into QuestDB. It converts the TradeBar fields into the required QuestDB
    /// column types like f64 and TimestampNanos.
    ///
    /// The buffer is flushed based on the configured `max_buffer_size`. This
    /// allows efficiently inserting multiple trade bars in bulk.
    ///
    /// The `extract_nano_timestamp` and `convert_decimal_to_f64` helper
    /// functions are used to do the field conversions from TradeBar to the
    /// QuestDB types.
    pub fn insert_trade_bars(
        &mut self,
        trade_bars: Vec<TradeBar>,
        table_name: &str,
        symbol: &str,
    ) -> QuestDBResult<()> {
        let sender = &mut self.sender;
        let max_len = trade_bars.len();
        let mut buffer = Buffer::with_max_name_len(max_len);

        let mut counter: usize = 0;
        let max_buffer_size = self.db_config.buffer_size();

        for trade_bar in trade_bars {
            counter += 1;

            let designated_timestamp = extract_nano_timestamp(&trade_bar);

            let price = convert_decimal_to_f64(&trade_bar.price());

            let volume = convert_decimal_to_f64(&trade_bar.volume());

            buffer
                .table(table_name)
                .expect("Failed to set table name")
                .symbol("symbol", symbol)
                .expect("Failed to set symbol")
                .column_f64("price", price)
                .expect("Failed to set price")
                .column_f64("volume", volume)
                .expect("Failed to set volume")
                .at(designated_timestamp)
                .expect("Failed to set timestamp");

            if counter == max_buffer_size {
                // Add multiple rows before flushing.
                // It's recommended to keep a timer and/or a buffer size before flushing.
                sender.flush(&mut buffer).expect("Failed to flush buffer");

                // restart counter
                counter = 0;
            }
        }

        // Flush out all the rest.
        sender.flush(&mut buffer).expect("Failed to flush buffer");

        Ok(())
    }
}

fn extract_nano_timestamp(trade_bar: &TradeBar) -> TimestampNanos {
    let nanos = trade_bar
        .date_time()
        .timestamp_nanos_opt()
        .expect("Failed to convert UTC timestamp into nanoseconds");

    TimestampNanos::new(nanos)
}

fn convert_decimal_to_f64(decimal: &rust_decimal::Decimal) -> f64 {
    decimal.to_f64().expect("Failed to convert decimal to f64")
}

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
    /// Creates a new QuestDBManager instance.
    ///
    /// # Arguments
    ///
    /// * `db_config` - The DBConfig with QuestDB connection details.
    ///
    /// # Returns
    ///
    /// Returns a new QuestDBManager instance.
    ///
    /// # Functionality
    ///
    /// This function extracts the host and port from the provided DBConfig.
    /// It uses these to create a new Sender via the SenderBuilder.
    /// The Sender is used to insert data into QuestDB via the ILP protocol.
    ///
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
    /// Inserts trade bars into QuestDB.
    ///
    /// # Arguments
    ///
    /// * `trade_bars` - The vector of trade bars to insert.
    /// * `table_name` - The name of the table to insert the trade bars into.
    /// * `symbol` - The symbol the trade bars are for.
    /// * `symbol_id` - The numeric id of the symbol.
    /// * `symbol_table_name` - The name of the symbol metadata table.
    ///
    /// # Returns
    ///
    /// Returns a `QuestDBResult<()>` indicating success or failure.
    ///
    /// # Functionality
    ///
    /// This function takes a vector of `TradeBar` structs and inserts them into the
    /// specified QuestDB table. It also inserts a metadata record into a separate
    /// symbol table with the symbol name, id, number of rows inserted, and destination
    /// table name.
    ///
    /// The trade bars are inserted in batches based on the configured buffer size.
    /// Timestamps are extracted from the `TradeBar` and converted to nanoseconds.
    /// Price and volume decimal values are converted to `f64`.
    ///
    ///
    /// # Arguments
    ///
    /// * `trade_bars` - The vector of trade bars to insert.
    /// * `table_name` - The name of the table to insert the trade bars into.
    /// * `symbol` - The symbol the trade bars are for.
    /// * `symbol_id` - The numeric id of the symbol.
    /// * `symbol_table_name` - The name of the symbol metadata table.
    ///
    /// # Returns
    ///
    /// Returns a `QuestDBResult<()>` indicating success or failure.
    ///
    /// # Functionality
    ///
    /// This function takes a vector of `TradeBar` structs and inserts them into the
    /// specified QuestDB table. It also inserts a metadata record into a separate
    /// symbol table with the symbol name, id, number of rows inserted, and destination
    /// table name.
    ///
    /// The trade bars are inserted in batches based on the configured buffer size.
    /// Timestamps are extracted from the `TradeBar` and converted to nanoseconds.
    /// Price and volume decimal values are converted to `f64`.
    ///
    pub fn insert_trade_bars(
        &mut self,
        trade_bars: Vec<TradeBar>,
        table_name: &str,
        symbol: &str,
        symbol_id: i64,
        meta_data_table: &str,
    ) -> QuestDBResult<()> {
        // Determine the total number of rows to insert into the trade table.
        let number_of_rows = trade_bars.len();

        // Acquire a mut reference to the sender.
        let sender = &mut self.sender;

        // Create a buffer with the size equal to the maximum number of rows.
        let mut buffer = Buffer::with_max_name_len(number_of_rows);

        // The maximum buffer size (number of buffered rows) is used to determine when to flush the buffer.
        let max_buffer_size = self.db_config.buffer_size();

        // Counter increments for each row inserted into the buffer until it hits the maximum buffer size.
        let mut counter: usize = 0;

        for trade_bar in trade_bars {
            counter += 1;

            let designated_timestamp = extract_nano_timestamp(&trade_bar);
            let price = convert_decimal_to_f64(&trade_bar.price());
            let volume = convert_decimal_to_f64(&trade_bar.volume());

            buffer
                .table(table_name)
                .expect("Failed to set table name")
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

        // Flush out all the remaining trade bars.
        sender.flush(&mut buffer).expect("Failed to flush buffer");

        buffer
            .table(meta_data_table)
            .expect("Failed to set symbol table name")
            .symbol("symbol", symbol)
            .expect("Failed to set symbol")
            .column_i64("symbol_id", symbol_id)
            .expect("Failed to set symbol_id")
            .column_i64("number_of_rows", number_of_rows as i64)
            .expect("Failed to set number_of_rows")
            .column_str("table_name", table_name)
            .expect("Failed to set trade bars table_name")
            .at(TimestampNanos::now())
            .expect("Failed to set timestamp");

        // Flush out the symbol table record.
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

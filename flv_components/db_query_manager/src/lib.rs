mod error;
mod query_ohlcv;
mod query_symbols;
mod query_trades;
mod query_utils;

use common::prelude::DBConfig;
use tokio_postgres::{Client, NoTls};

pub struct QueryDBManager {
    client: Client,
    // connection: Connection<Socket, NoTlsStream>
}

impl QueryDBManager {
    /// Creates a new QueryDBManager instance.
    ///
    /// # Arguments
    ///
    /// * `db_config` - The database configuration containing connection parameters.
    ///
    /// # Returns
    ///
    /// A new QueryDBManager instance connected to the database.
    ///
    /// # Errors
    ///
    /// Will return an error if the connection to the database fails.
    ///
    /// # Example
    ///
    /// ```rust
    /// use common::prelude::DBConfig;
    /// use db_query_manager::QueryDBManager;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///  let db_config =  DBConfig::new(9009, "0.0.0.0".into());
    ///  let query_manager = QueryDBManager::new(db_config).await.expect("Failed to create db connection");
    /// }
    /// ```
    ///
    pub async fn new(db_config: DBConfig) -> Result<Self, tokio_postgres::Error> {
        {
            // Extract connection string.
            let params = db_config.pg_connection_string();

            // Connect to the database.
            let (client, connection) = tokio_postgres::connect(&params, NoTls)
                .await
                .expect("Failed to connect to DB");

            // The connection object performs the actual communication with the database,
            // so spawn it off to run on its own.
            tokio::spawn(async move {
                if let Err(e) = connection.await {
                    eprintln!("connection error: {}", e);
                }
            });

            Ok(Self { client })
        }
    }
}

impl QueryDBManager {
    /// Checks if the database connection is closed.
    ///
    /// # Returns
    ///
    /// Returns `true` if the connection is closed, `false` otherwise.
    ///
    /// # Example
    /// ```rust
    /// use common::prelude::DBConfig;
    /// use db_query_manager::QueryDBManager;
    ///
    /// #[tokio::main]
    /// async fn main() {
    /// let db_config =  DBConfig::new(9009, "0.0.0.0".into());
    /// let query_manager = QueryDBManager::new(db_config).await.expect("Failed to create db connection");
    ///
    /// if query_manager.is_close().await {
    ///         println!("Connection is closed: {}", query_manager.is_close().await);
    /// } else {
    ///     // Connection is open
    /// }
    /// }
    /// ```
    ///
    pub async fn is_close(&self) -> bool {
        self.client.is_closed()
    }
}

pub mod error;
mod query_ohlcv;
mod query_symbols;
mod query_trades;
mod query_utils;

use common::prelude::DBConfig;
use deadpool_postgres::{Config, CreatePoolError, ManagerConfig, Pool, RecyclingMethod, Runtime};
use tokio_postgres::NoTls;

pub struct QueryDBManager {
    pool: Pool,
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
    pub async fn new(db_config: DBConfig) -> Result<Self, CreatePoolError> {

        // Configure the connection pool
        let mut cfg = Config::new();
        cfg.host = Some(db_config.host().to_string());
        cfg.dbname = Some(db_config.pg_database().to_string());
        cfg.user = Some(db_config.pg_user().to_string());
        cfg.password = Some(db_config.pg_password().to_string());
        cfg.manager = Some(ManagerConfig { recycling_method: RecyclingMethod::Fast });

        // Create new QueryDBManager with the configured connection pool.
        return match cfg.create_pool(Some(Runtime::Tokio1), NoTls) {
            Ok(pool) => Ok(Self { pool }),
            Err(e) => Err(e),
        };
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
    ///  let db_config =  DBConfig::new(9009, "0.0.0.0".into());
    /// let query_manager = QueryDBManager::new(db_config).await.expect("Failed to create db connection");
    ///
    /// let open = query_manager.is_close().await;
    ///
    /// if open{
    ///         println!("Connection is closed: {}", open);
    /// } else {
    ///     println!("Connection is open: {}", open);
    /// }
    /// }
    /// ```
    ///
    pub async fn is_close(&self) -> bool {
        // Get a client from the connection pool
        let client = self
            .pool
            .get()
            .await
            .expect("[QueryDBManager/is_close]: Failed to get client from connection pool");

        client.is_closed()
    }
}

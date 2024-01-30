pub mod error;
mod query_ohlcv;
mod query_symbols;
mod query_trades;
mod query_utils;

use common::prelude::DBConfig;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::fmt::Error;

const FN_NAME: &str = "[QueryDBManager]:";

pub struct QueryDBManager {
    pool: Pool<Postgres>,
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
    ///   // Run Queries
    ///
    ///   // Close the connection pool
    ///   query_manager.close().await;
    /// }
    /// ```
    ///
    pub async fn new(db_config: DBConfig) -> Result<Self, Error> {
        let url = db_config.pg_connection_url();
        let max_connections = db_config.pg_max_connections();
        let pool = create_connection_pool(url, max_connections).await;

        Ok(Self { pool })
    }
}

/// Creates a connection pool to the Postgres database.
///
/// # Arguments
///
/// * `url` - The database URL
/// * `max_connections` - The maximum number of connections in the pool
///
/// # Returns
///
/// A Pool of Postgres connections.
///
/// # Errors
///
/// Returns a connection pool error if connecting to the database fails.
///
async fn create_connection_pool(url: String, max_connections: u32) -> Pool<Postgres> {
    // Create a connection pool to the database
    // https://github.com/questdb/questdb/issues/3204
    let pool_connection = PgPoolOptions::new()
        .max_connections(max_connections)
        .connect(&url)
        .await;

    // Check if the connection to the database was successful
    match pool_connection {
        Ok(pool) => {
            println!("✅ Database Connection OK!");
            pool
        }
        Err(err) => {
            panic!("{FN_NAME} ❌ Database Connection FAILED ❌: {:?}", err);
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
    ///  let db_config =  DBConfig::new(9009, "0.0.0.0".into());
    /// let query_manager = QueryDBManager::new(db_config).await.expect("Failed to create db connection");
    ///
    /// let open = query_manager.is_close().await;
    ///
    /// if open{
    ///         println!("✅ DB Connection is open: {}", open);
    /// } else {
    ///     println!("❌ DB Connection is closed");
    /// }
    ///
    ///   // Close the connection pool
    ///   query_manager.close().await;
    /// }
    /// ```
    ///
    pub async fn is_close(&self) -> bool {
        self.pool.is_closed()
    }

    /// Closes the database connection pool.
    ///
    /// # Arguments
    ///
    /// * `self` - The QueryManager instance
    ///
    /// # Returns
    ///
    /// Result with the following outcomes:
    ///
    /// - Ok(()) - The connection pool was closed successfully.
    /// - Err(e) - An error occurred while closing the connection pool.
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
    ///
    ///   query_manager.close().await;
    ///
    ///  }
    /// ```
    pub async fn close(&self) {
        self.pool.close().await
    }
}

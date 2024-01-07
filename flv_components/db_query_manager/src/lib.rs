mod error;
mod query_symbols;
mod query_utils;

use common::prelude::DBConfig;
use postgres::{Client, NoTls};

// FIXME: PG connection needs to be fully tokio async to work properly in QDGW service.
// https://docs.rs/tokio-postgres/latest/tokio_postgres/

pub struct QueryDBManager {
    client: Client,
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
    /// ```
    /// use common::prelude::DBConfig;
    /// use db_query_manager::QueryDBManager;
    ///
    ///  let db_config =  DBConfig::new(9009, "0.0.0.0".into());
    /// let query_manager = QueryDBManager::new(db_config);
    /// ```
    pub fn new(db_config: DBConfig) -> Self {
        let params = db_config.pg_connection_string();

        let client = Client::connect(&params, NoTls).expect("Failed to connect to DB");

        Self { client }
    }
}

impl QueryDBManager {
    /// Closes the database connection.
    ///
    /// # Returns
    ///
    /// Returns nothing.
    ///
    /// # Errors
    ///
    /// Will return an error if closing the connection fails.
    ///
    /// # Example
    ///
    /// ```
    /// use common::prelude::DBConfig;
    /// use db_query_manager::QueryDBManager;
    ///
    /// let db_config =  DBConfig::new(9009, "0.0.0.0".into());
    /// let mut query_manager = QueryDBManager::new(db_config);
    ///
    /// // Use query_manager to query the database
    ///
    /// query_manager.close().expect("Failed to close DB connection"); // Close connection when done
    /// ```
    pub fn close(self) -> Result<(), postgres::error::Error> {
        self.client.close()
    }

    /// Checks if the database connection is closed.
    ///
    /// # Returns
    ///
    /// Returns `true` if the connection is closed, `false` otherwise.
    ///
    /// # Example
    ///
    /// ```
    /// use common::prelude::DBConfig;
    /// use db_query_manager::QueryDBManager;
    ///
    /// let db_config =  DBConfig::new(9009, "0.0.0.0".into());
    /// let query_manager = QueryDBManager::new(db_config);
    ///
    /// if query_manager.is_close() {
    ///         println!("Connection is closed: {}", query_manager.is_close());
    /// } else {
    ///     // Connection is open
    /// }
    /// ```
    pub fn is_close(&self) -> bool {
        self.client.is_closed()
    }
}

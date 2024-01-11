use std::fmt;
use std::fmt::{Debug, Display, Formatter};

use serde::{Deserialize, Serialize};

const BUFFER_SIZE: usize = 50_000;

/// DBConfig represents the configuration for connecting to a database instance.
///
/// # Fields
/// * `port`: The port number to connect to at the server host. The default port is 9009.
/// * `host`: The DNS resolvable name of the host to connect to. Exactly one of `host` and `address`
#[derive(Debug, Eq, Clone, PartialEq, Serialize, Deserialize)]
pub struct DBConfig {
    /// Port number to connect to at the server host. The default port for the line protocol is 9009.
    port: u16,
    /// DNS resolvable name of the host to connect to.
    host: String,
    /// Name of the database table containing all the exchanges.
    exchange_table_name: String,
    /// ILP Buffer size before flushing to the server.
    buffer_size: usize,
    //
    // authentication is not yet supported.
    // see:https://github.com/questdb/c-questdb-client/blob/main/questdb-rs/examples/auth.rs
    // kid: String,
    // d: String,
    // x: String,
    // y: String,
}

impl DBConfig {
    /// Creates a new DBConfig instance.
    ///
    /// # Arguments
    ///
    /// * `port` - The port number to connect to the database
    /// * `host` - The hostname of the database server
    /// * `exchange_table_name` - The name of the exchange table in the database
    ///
    /// # Returns
    ///
    /// A new DBConfig instance with the provided parameters and a default buffer size.
    ///
    /// # Example
    ///
    /// ```rust
    /// use common::prelude::DBConfig;
    ///
    /// let config = DBConfig::new(9009, "localhost".to_string(), "exchanges".to_string());
    /// ```
    pub fn new(port: u16, host: String, exchange_table_name: String) -> Self {
        Self {
            port,
            host,
            exchange_table_name,
            buffer_size: BUFFER_SIZE,
        }
    }
}

// getters
impl DBConfig {
    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn host(&self) -> &String {
        &self.host
    }

    pub fn exchange_table_name(&self) -> &str {
        &self.exchange_table_name
    }

    pub fn buffer_size(&self) -> usize {
        self.buffer_size
    }

    pub fn pg_connection_string(&self) -> String {
        // https://questdb.io/docs/develop/query-data/#postgresql-wire-protocol
        format!(
            "user=admin password=quest host={} port=8812 dbname=qdb",
            self.host
        )
    }
}

impl Display for DBConfig {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "DBConfig {{ port: {}, host: {}, exchange_table_name: {} }}",
            self.port, self.host, self.exchange_table_name
        )
    }
}

use std::fmt;
use std::fmt::{Debug, Display, Formatter};

use serde::{Deserialize, Serialize};

const BUFFER_SIZE: usize = 50_000;

/// Configuration for the QuestDB database.
/// Requires both, ILP and Postgres connection parameters.
/// Default ports are used for the ILP and Postgres connections.
///
/// * ILP: 9009
/// * Postgres Wire: 8812
///
/// See <https://questdb.io/docs/develop/connect/>`
#[derive(Debug, Eq, Clone, PartialEq, Serialize, Deserialize)]
pub struct DBConfig {
    /// ILP ort number to connect to at the server host. The default port for the line protocol is 9009.
    port: u16,
    /// DNS resolvable name of the host to connect to.
    host: String,
    /// ILP Buffer size before flushing to the server.
    buffer_size: usize,
    // Postgres authentication parameters.
    /// Postgres username.
    pg_user: String,
    /// Postgres password.
    pg_password: String,
    /// Postgres database name.
    pg_database: String,
    /// Postgres server address.
    pg_port: u16,
    /// Postgres max connections.
    pg_max_connections: u32,
    // Secure authentication is not implemented.
    // See: https://github.com/questdb/c-questdb-client/blob/main/questdb-rs/examples/auth.rs
}

impl DBConfig {
    /// Creates a new DBConfig instance.
    ///
    /// # Arguments
    ///
    /// * `port`: The ILP port number to connect to at the server host. The default port is 9009.
    /// * `host` - The hostname of the database server
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
    /// let config = DBConfig::new(9009, "localhost".to_string());
    /// ```
    pub fn new(port: u16, host: String) -> Self {
        Self {
            port,
            host,
            buffer_size: BUFFER_SIZE,
            // Default Postgres authentication parameters.
            // https://questdb.io/docs/develop/connect/
            pg_user: "admin".to_string(),
            pg_password: "quest".to_string(),
            pg_database: "qdb".to_string(),
            pg_port: 8812,
            pg_max_connections: 10,
        }
    }

    /// Creates a new DBConfig instance configured for Postgres.
    ///
    /// This allows providing Postgres specific configuration like credentials and database name.
    ///
    /// # Arguments
    ///
    /// * `port`: The ILP port number to connect to at the server host. The default port is 9009.
    /// * `host` - The QuestDB host address
    /// * `pg_user` - The Postgres user to authenticate with. Default is "admin"
    /// * `pg_password` - The password for the Postgres user
    /// * `pg_database` - The name of the Postgres database to use. Default is "qdb"
    /// * `pg_port` - The port to connect to Postgres on. Default is 8812
    /// * `pg_max_connections` - The maximum number of connections to Postgres. Default is 10
    ///
    /// # Returns
    ///
    /// A DBConfig instance configured with the provided Postgres parameters.
    ///
    /// # Example
    ///
    /// ```
    /// use common::prelude::DBConfig;
    ///
    /// let config = DBConfig::new_with_pg_config(
    ///     9009, "localhost".into(), "myuser".into(), "password".into(),
    ///     "qdb".into(), 8812, 10);
    /// ```
    ///
    pub fn new_with_pg_config(
        port: u16,
        host: String,
        pg_user: String,
        pg_password: String,
        pg_database: String,
        pg_port: u16,
        pg_max_connections: u32,
    ) -> Self {
        Self {
            port,
            host,
            buffer_size: BUFFER_SIZE,
            pg_user,
            pg_password,
            pg_database,
            pg_port,
            pg_max_connections,
        }
    }
}

impl DBConfig {
    /// Generates a PostgreSQL connection string from the DBConfig.
    ///
    /// The connection string contains the parameters required to connect to
    /// QuestDB's PostgreSQL endpoint, including:
    ///
    /// - user
    /// - password
    /// - host
    /// - port
    /// - dbname
    ///
    /// The string follows the format expected by PostgreSQL clients.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use common::prelude::DBConfig;
    ///
    /// let config = DBConfig::new(9009, "localhost".into());
    /// let conn_str = config.pg_connection_string();
    ///
    /// assert_eq!(conn_str, "user=admin password=quest host=localhost port=8812 dbname=qdb");
    /// ```
    ///
    pub fn pg_connection_string(&self) -> String {
        // https://questdb.io/docs/develop/query-data/#postgresql-wire-protocol
        format!(
            "user={} password={} host={} port={} dbname={}",
            self.pg_user, self.pg_password, self.host, self.pg_port, self.pg_database,
        )
    }

    pub fn pg_connection_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.pg_user, self.pg_password, self.host, self.pg_port, self.pg_database,
        )
    }
}

// getters
impl DBConfig {
    /// Returns the configured ILP port number.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use common::prelude::DBConfig;
    ///
    /// let config = DBConfig::new(9009, "localhost".into());
    /// assert_eq!(config.port(), 9009);
    /// ```
    pub fn port(&self) -> u16 {
        self.port
    }

    /// Returns the configured host string.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use common::prelude::DBConfig;
    ///
    /// let config = DBConfig::new(9009, "localhost".into());
    /// assert_eq!(config.host(), "localhost");
    /// ```
    pub fn host(&self) -> &str {
        &self.host
    }

    /// Returns the configured buffer size.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use common::prelude::DBConfig;
    ///
    /// let config = DBConfig::new(9009, "localhost".into());
    /// assert_eq!(config.buffer_size(), 50000);
    /// ```
    pub fn buffer_size(&self) -> usize {
        self.buffer_size
    }

    /// Returns the configured Postgres user.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use common::prelude::DBConfig;
    ///
    /// let config = DBConfig::new_with_pg_config(
    ///         27017,
    ///         "localhost".to_string(),
    ///         "pguser".to_string(),
    ///         "pgpass".to_string(),
    ///         "pgdb".to_string(),
    ///         5432,
    ///         10,
    ///     );
    ///
    /// assert_eq!(config.pg_user(), "pguser");
    /// ```
    pub fn pg_user(&self) -> &str {
        &self.pg_user
    }

    /// Returns the configured Postgres password.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use common::prelude::DBConfig;
    ///
    /// let config = DBConfig::new_with_pg_config(
    ///         27017,
    ///         "localhost".to_string(),
    ///         "pguser".to_string(),
    ///         "pgpass".to_string(),
    ///         "pgdb".to_string(),
    ///         5432,
    ///         10,
    ///     );
    ///
    /// assert_eq!(config.pg_password(), "pgpass");
    /// ```
    pub fn pg_password(&self) -> &str {
        &self.pg_password
    }

    /// Returns the configured Postgres database name.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use common::prelude::DBConfig;
    ///
    /// let config = DBConfig::new_with_pg_config(
    ///         27017,
    ///         "localhost".to_string(),
    ///         "pguser".to_string(),
    ///         "pgpass".to_string(),
    ///         "pgdb".to_string(),
    ///         5432,
    ///         10,
    ///     );
    ///
    /// assert_eq!(config.pg_database(), "pgdb");
    /// ```
    pub fn pg_database(&self) -> &str {
        &self.pg_database
    }

    /// Returns the configured Postgres port number.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use common::prelude::DBConfig;
    ///
    /// let config = DBConfig::new_with_pg_config(
    ///         27017,
    ///         "localhost".to_string(),
    ///         "pguser".to_string(),
    ///         "pgpass".to_string(),
    ///         "pgdb".to_string(),
    ///         5432,
    ///         10,
    ///     );
    ///
    /// assert_eq!(config.pg_port(), 5432);
    /// ```
    pub fn pg_port(&self) -> u16 {
        self.pg_port
    }

    pub fn pg_max_connections(&self) -> u32 {
        self.pg_max_connections
    }
}

impl Display for DBConfig {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "DBConfig {{\n  port: {},\n  host: {},\n  pg_user: {},\n  pg_database: {}\n pg_port: {}\n}}",
            self.port,
            self.host,
            self.pg_user,
            self.pg_database,
            self.pg_port,
        )
    }
}

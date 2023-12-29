use std::fmt::{Debug, Display, Formatter};

use serde::{Deserialize, Serialize};

/// DBConfig represents the configuration for connecting to a Memgraph instance.
///
/// # Fields
///
/// * `port`: The port number to connect to at the server host. The default port is 7687.
/// * `host`: The DNS resolvable name of the host to connect to. Exactly one of `host` and `address`
/// parameters must be specified.
/// * `username`: The username to connect as.
/// * `password`: The password to be used if the server demands password authentication.
/// * `client_name`: The alternate name and version of the client to send to the server. The default
/// is "MemgraphBolt/0.1".
#[derive(Debug, Eq, Clone, PartialEq, Serialize, Deserialize)]
pub struct DBConfig {
    /// Port number to connect to at the server host. The default port is 7687.
    port: u16,
    /// DNS resolvable name of the host to connect to.
    host: String,
    /// DB name to connect to.
    db_name: String,
    /// DB namespace to connect to.
    db_namespace: String,
    /// Username to connect as.
    username: String,
    /// Password to be used if the server demands password authentication.
    password: String,
    /// Alternate name and version of the client to send to server. The default is "MemgraphBolt/0.1".
    client_name: String,
}

impl DBConfig {
    pub fn new(
        port: u16,
        host: String,
        db_name: String,
        db_namespace: String,
        username: String,
        password: String,
        client_name: String,
    ) -> Self {
        Self {
            port,
            host,
            db_name,
            db_namespace,
            username,
            password,
            client_name,
        }
    }

    /// Creates a new unsafe config for a connection without authentication.
    pub fn new_connection(port: u16, host: String) -> Self {
        Self {
            port,
            host,
            ..Default::default()
        }
    }

    /// Creates a new config for authentication.
    pub fn new_authentication(username: String, password: String) -> Self {
        Self {
            username,
            password,
            ..Default::default()
        }
    }

    /// Creates a new config for a connection with authentication.
    pub fn new_connection_with_authentication(
        port: u16,
        host: String,
        username: String,
        password: String,
    ) -> Self {
        Self {
            port,
            host,
            username,
            password,
            ..Default::default()
        }
    }
}

impl Default for DBConfig {
    /// Returns the default configuration.
    //https://surrealdb.com/docs/installation/running/docker
    fn default() -> Self {
        Self {
            port: 8000,
            host: "0.0.0.0".to_string(),
            db_name: "test".to_string(),
            db_namespace: "test".to_string(),
            username: "root".to_string(),
            password: "root".to_string(),
            client_name: String::from("dbgw"),
        }
    }
}

// getters
impl DBConfig {
    /// Returns the name of the database to connect to.
    pub fn db_name(&self) -> &String {
        &self.db_name
    }

    /// Returns the db namespace to connect to.
    pub fn db_namespace(&self) -> &String {
        &self.db_namespace
    }

    /// Returns the port number to connect to at the server host.
    pub fn port(&self) -> u16 {
        self.port
    }

    /// Returns the DNS resolvable name of the host to connect to.
    pub fn host(&self) -> &String {
        &self.host
    }

    /// Returns the username to connect as.
    pub fn username(&self) -> &String {
        &self.username
    }

    /// Returns the password to be used if the server demands password authentication.
    pub fn password(&self) -> &String {
        &self.password
    }

    /// Returns the alternate name and version of the client to send to server.
    pub fn client_name(&self) -> &str {
        &self.client_name
    }
}

impl Display for DBConfig {
    /// Formats the config as a string.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "DBConfig {{ port: {}, host: {}, db_name: {}, db_namespace: {}, username: {},\
                password: {}, client_name: {} }}",
            self.port,
            self.host,
            self.db_name,
            self.db_namespace,
            self.username,
            self.password,
            self.client_name
        )
    }
}

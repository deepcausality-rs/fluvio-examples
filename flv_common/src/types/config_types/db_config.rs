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
}

impl DBConfig {
    pub fn new(
        port: u16,
        host: String,
        db_name: String,
        db_namespace: String,
        username: String,
        password: String,
    ) -> Self {
        Self {
            port,
            host,
            db_name,
            db_namespace,
            username,
            password,
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
}

impl Display for DBConfig {
    /// Formats the config as a string.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "DBConfig {{ port: {}, host: {}, db_name: {}, db_namespace: {}, username: {},\
                password: {} }}",
            self.port,
            self.host,
            self.db_name,
            self.db_namespace,
            self.username,
            self.password,
        )
    }
}

use std::fmt::{Debug, Display, Formatter};

use serde::{Deserialize, Serialize};

/// DBConfig represents the configuration for connecting to a Memgraph instance.
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
    //
    // authentication is not yet supported.
    // see:https://github.com/questdb/c-questdb-client/blob/main/questdb-rs/examples/auth.rs
    // kid: String,
    // d: String,
    // x: String,
    // y: String,
}

impl DBConfig {
    pub fn new(port: u16, host: String) -> Self {
        Self { port, host }
    }
}

// getters
impl DBConfig {
    /// Returns the port number to connect to at the server host.
    pub fn port(&self) -> u16 {
        self.port
    }

    /// Returns the DNS resolvable name of the host to connect to.
    pub fn host(&self) -> &String {
        &self.host
    }
}

impl Display for DBConfig {
    /// Formats the config as a string.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "DBConfig {{ port: {}, host: {} }}", self.port, self.host)
    }
}

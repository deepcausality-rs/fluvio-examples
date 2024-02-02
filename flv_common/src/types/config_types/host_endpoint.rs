use std::fmt::{Display, Formatter};

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct HostEndpoint {
    /// Host URI.
    host_uri: String,
    /// Port number.
    port: u16,
}

impl HostEndpoint {
    pub fn new(host_uri: String, port: u16) -> Self {
        Self { host_uri, port }
    }
}

impl HostEndpoint {
    pub fn host_uri(&self) -> &str {
        &self.host_uri
    }
    pub fn port(&self) -> u16 {
        self.port
    }
}

impl Display for HostEndpoint {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "host_uri: {},  port: {}", self.host_uri, self.port)
    }
}

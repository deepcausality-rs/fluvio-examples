mod error;
mod lookup;
mod utils_proto;

use common::prelude::HostEndpoint;
use proto::binding::symdb_service_client::SymdbServiceClient;
use std::fmt::Error;
use std::sync::{Arc, RwLock};
use tonic::transport::{Channel, Uri};

/// Client for interacting with the SymdbService.
///
/// Wraps a SymdbServiceClient and provides methods to
/// lookup symbols, symbol IDs, and exchange names.
///
#[derive(Debug, Clone)]
pub struct SymdbClient {
    client: Arc<RwLock<SymdbServiceClient<Channel>>>,
}

impl SymdbClient {
    /// Creates a new SymdbClient instance.
    ///
    /// # Arguments
    ///
    /// * `config: HostEndpoint` - The endpoint configuration of the SYMDB Service gRPC server
    ///
    /// # Returns
    ///
    /// Returns a SymdbClient connected to the given address.
    ///
    /// # Example
    ///
    /// ```no_run
    /// #[tokio::main]
    /// async fn main() {
    /// use symdb_client::SymdbClient;
    /// use common::prelude::HostEndpoint;
    ///
    /// let config = HostEndpoint::new("127.0.0.1".to_string(), 7070);
    /// let client = SymdbClient::new(config).await?;
    /// }
    /// ```
    pub async fn new(config: HostEndpoint) -> Result<Self, Error> {
        // Extract host and port from config
        let port = config.port();
        let host = config.host_uri();

        // "http://[::1]:7070"
        let s = format!("http://{}:{}", host, port);
        let uri = s
            .parse::<Uri>()
            .unwrap_or_else(|_| panic!("[SymdbClient]: Failed to parse server URI: {}", s));

        // println!("[SymdbClient]: Server URI: {}", &s);

        // creating a channel that connects to server
        let channel = Channel::builder(uri)
            .connect()
            .await
            .unwrap_or_else(|_| panic!("\r\n ❌[SymdbClient]: Failed to connect to SYMDB service on: {} \r\n  \r\n Detail: \r\n", s));

        let client = SymdbServiceClient::new(channel);

        let client = Arc::new(RwLock::new(client));

        Ok(Self { client })
    }
}

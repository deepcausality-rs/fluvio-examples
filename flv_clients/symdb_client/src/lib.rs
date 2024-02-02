mod error;
mod lookup;
mod utils_proto;

use tonic::transport::{Channel, Uri};
use common::prelude::HostEndpoint;
use proto::binding::symdb_service_client::SymdbServiceClient;

#[derive(Debug, Clone)]
pub struct SymdbClient{
    client: SymdbServiceClient<Channel>
}

impl SymdbClient {
    pub async fn new(config: HostEndpoint) -> Self {

        let port = config.port();
        let host = config.host_uri();

        // "http://[::1]:50051"
        let s = format!("http://{}:{}", host, port);
        let uri = s
            .parse::<Uri>()
            .unwrap_or_else(|_| panic!("DBGatewayClient: Failed to parse server URI: {}", s));

        // println!("DBGatewayClient: Server URI: {}", &s);

        // creating a channel that connects to server
        let channel = Channel::builder(uri)
            .connect()
            .await
            .unwrap_or_else(|_| panic!("\r\n DBGatewayClient: Failed to connect to DBGW service on: {} \r\n  \r\n Detail: \r\n", s));

        let client = SymdbServiceClient::new(channel);

        Self { client }
    }
}
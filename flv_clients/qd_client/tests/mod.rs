use common::prelude::{IggyConfig, IggyUser};
use qd_client::QDClient;

const CLIENT_ID: u16 = 77;

//
// Requires that the QDGW server is running on localhost
// Start the server with:
//
// cargo run --bin qdgw
//

#[tokio::test]
async fn test_new() {
    let user = IggyUser::default();
    let client_config = IggyConfig::from_client_id(user, CLIENT_ID as u32, 50000, false);

    // Happy path
    let client = QDClient::new(CLIENT_ID, client_config).await;
    assert!(client.is_ok());

    let client = client.unwrap();
    assert_eq!(client.client_id(), CLIENT_ID);

    client.close().await.expect("Failed to close client");
}

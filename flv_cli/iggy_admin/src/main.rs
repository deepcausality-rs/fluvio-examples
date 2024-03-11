use std::error::Error;

use iggy::client::UserClient;
use iggy::clients::client::IggyClient;
use iggy::users::login_user::LoginUser;
use iggy::users::logout_user::LogoutUser;

use common::prelude::{IggyConfig, IggyUser};

const CLIENT_ID: u16 = 1;

#[tokio::main]
async fn main() {
    let user = IggyUser::default();
    let config = IggyConfig::from_client_id(user.clone(), CLIENT_ID as u32, 50000, false);

    let client = iggy_utils::get_iggy_client("127.0.0.1:8090".to_string())
        .await
        .expect("Failed to build iggy client");

    init(&client, &user).await.expect("Failed to init");

    create_new_user(&client)
        .await
        .expect("Failed to create new user");

    cleanup(&client, &config).await.expect("Failed to cleanup");
}

async fn init(client: &IggyClient, user: &IggyUser) -> Result<(), Box<dyn Error>> {
    iggy_utils::init_consumer(&client, user)
        .await
        .expect("Failed to init");

    Ok(())
}

async fn cleanup(client: &IggyClient, iggy_config: &IggyConfig) -> Result<(), Box<dyn Error>> {
    // Delete stream and topic before shutting down.
    iggy_utils::cleanup(&client, &iggy_config)
        .await
        .expect("Failed to clean up iggy consumer");

    // Logout user. Call it just once as consumer and producer use the same user.
    iggy_utils::logout_user(&client)
        .await
        .expect("Failed to logout user");

    // Shutdown consumer
    iggy_utils::shutdown(&client)
        .await
        .expect("Failed to shutdown iggy consumer");

    Ok(())
}

async fn create_new_user(client: &IggyClient) -> Result<IggyUser, Box<dyn Error>> {
    let new_user = IggyUser::new("qdgw", "qdgw");

    println!("Creating new user...");
    iggy_utils::create_user(&client, &new_user)
        .await
        .expect("Failed to create new user");

    println!("Login new user...");
    client
        .login_user(&LoginUser {
            username: new_user.username().to_string(),
            password: new_user.password().to_string(),
        })
        .await
        .expect("Failed to login user");

    println!("Creating new token...");
    let token = iggy_utils::create_token(&client, "qdgw_token".to_string())
        .await
        .expect("Failed to create token");

    println!("Token: {}", token);

    client
        .logout_user(&LogoutUser {})
        .await
        .expect("Failed to logout user");

    Ok(new_user)
}

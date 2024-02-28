use iggy::clients::client::{IggyClient, IggyClientBuilder};
use iggy::error::IggyError;
use std::error::Error;
use iggy::users::logout_user::LogoutUser;
use iggy::users::login_user::LoginUser;
use iggy::client::{Client, UserClient};
use crate::types::User;

fn get_tcp_server_addr() -> String {
   "127.0.0.1:8090".to_string()
}


/// `get_client` is used to create a new client to interact with the server.
pub(crate) async fn get_client() -> Result<IggyClient, IggyError> {
    IggyClientBuilder::new()
        .with_tcp()
        .with_server_address(get_tcp_server_addr())
        .build()
}

/// `init` command is used to login to the server.
pub(crate) async fn init(client: &IggyClient, user: &User) -> Result<(), Box<dyn Error>> {
    client
        .connect()
        .await
        .expect("Failed to connect to iggy server");

    match client
        .login_user(&LoginUser {
            username: user.username().to_string(),
            password: user.password().to_string(),
        }).await {
        Ok(_) => println!("User logged in."),
        Err(_) => println!("User already logged in."),
    }

    Ok(())
}


/// `shutdown` command is used to logout and disconnect from the server.
pub(crate) async fn shutdown(client: &IggyClient) -> Result<(), Box<dyn Error>> {
    match client
        .logout_user(&LogoutUser {})
        .await {
        Ok(_) => println!("User logged out."),
        Err(_) => println!("User was already logged out."),
    }

    client
        .disconnect()
        .await
        .expect("Failed to connect to iggy server");

    Ok(())
}

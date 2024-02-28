use common::prelude::IggyUser;
use iggy::client::PersonalAccessTokenClient;
use iggy::client::{Client, UserClient};
use iggy::clients::client::{IggyClient, IggyClientBuilder};
use iggy::error::IggyError;
use iggy::models::user_status::UserStatus;
use iggy::personal_access_tokens::create_personal_access_token::CreatePersonalAccessToken;
use iggy::users::create_user::CreateUser;
use iggy::users::login_user::LoginUser;
use iggy::users::logout_user::LogoutUser;
use std::error::Error;

fn get_tcp_server_addr() -> String {
    "127.0.0.1:8090".to_string()
}

/// `get_client` is used to create a new client to interact with the server.
pub async fn get_iggy_client() -> Result<IggyClient, IggyError> {
    IggyClientBuilder::new()
        .with_tcp()
        .with_server_address(get_tcp_server_addr())
        .build()
}

/// `init` command is used to login to the server.
pub async fn init(client: &IggyClient, user: &IggyUser) -> Result<(), Box<dyn Error>> {
    client
        .connect()
        .await
        .expect("Failed to connect to iggy server");

    match client
        .login_user(&LoginUser {
            username: user.username().to_string(),
            password: user.password().to_string(),
        })
        .await
    {
        Ok(_) => println!("User logged in."),
        Err(_) => println!("User already logged in."),
    }

    Ok(())
}

/// `shutdown` command is used to logout and disconnect from the server.
pub async fn shutdown(client: &IggyClient) -> Result<(), Box<dyn Error>> {
    match client.logout_user(&LogoutUser {}).await {
        Ok(_) => println!("User logged out."),
        Err(_) => println!("User was already logged out."),
    }

    client
        .disconnect()
        .await
        .expect("Failed to connect to iggy server");

    Ok(())
}

/// `create_user` command is used to create a new user.
pub async fn create_user(client: &IggyClient, user: &IggyUser) -> Result<(), Box<dyn Error>> {
    match client
        .create_user(&CreateUser {
            username: user.username().to_string(),
            password: user.password().to_string(),
            status: UserStatus::Active,
            permissions: None,
        })
        .await
    {
        Ok(_) => println!("User created."),
        Err(_) => println!("User already exists."),
    }

    Ok(())
}

/// `create_token` command is used to create a new personal access token.
pub async fn create_token(
    client: &IggyClient,
    token_name: String,
) -> Result<String, Box<dyn Error>> {
    let token = match client
        .create_personal_access_token(&CreatePersonalAccessToken {
            name: token_name,
            expiry: None,
        })
        .await
    {
        Ok(raw_token) => raw_token.token,
        Err(err) => {
            println!("Error creating token: {}", err.as_string());
            return Err(err.into());
        }
    };

    Ok(token)
}

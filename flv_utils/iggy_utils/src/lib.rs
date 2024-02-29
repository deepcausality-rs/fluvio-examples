use common::prelude::{IggyConfig, IggyUser};
use iggy::client::{Client, UserClient};
use iggy::client::{PersonalAccessTokenClient, StreamClient, TopicClient};
use iggy::clients::client::{IggyClient, IggyClientBuilder};
use iggy::error::IggyError;
use iggy::models::user_status::UserStatus;
use iggy::personal_access_tokens::create_personal_access_token::CreatePersonalAccessToken;
use iggy::streams::create_stream::CreateStream;
use iggy::topics::create_topic::CreateTopic;
use iggy::users::create_user::CreateUser;
use iggy::users::login_user::LoginUser;
use iggy::users::logout_user::LogoutUser;
use std::error::Error;

fn get_tcp_server_addr() -> String {
    "127.0.0.1:8090".to_string()
}

/// Creates a new `IggyClient` instance and returns it.
///
/// # Returns
///
/// A `Result` type, which can either be an `Ok` variant containing a new `IggyClient` instance
/// or an `Err` variant containing a boxed dynamic error.
/// The `Ok` variant indicates that the `IggyClient` instance was successfully created,
/// while the `Err` variant indicates that there was an error while creating the instance.
///
pub async fn get_iggy_client() -> Result<IggyClient, IggyError> {
    IggyClientBuilder::new()
        .with_tcp()
        .with_server_address(get_tcp_server_addr())
        .build()
}

/// Initializes the connection to the Iggy server and logs in the user.
///
/// # Arguments
///
/// * `client` - A reference to an `IggyClient` instance, which is used to connect to the Iggy server and perform operations on it.
/// * `user` - A reference to an `IggyUser` instance, which contains the information about the user that is being logged in.
///
/// # Returns
///
/// A `Result` type, which can either be an `Ok` variant containing a unit value (`()`) or an `Err` variant containing a boxed dynamic error.
/// The `Ok` variant indicates that the user was successfully logged in, while the `Err` variant indicates that there was an error while logging in.
///
///
pub async fn init_client(client: &IggyClient, user: &IggyUser) -> Result<(), Box<dyn Error>> {
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

pub async fn init_producer(
    client: &IggyClient,
    iggy_config: &IggyConfig,
) -> Result<(), Box<dyn Error>> {
    match client
        .create_stream(&CreateStream {
            stream_id: Some(iggy_config.stream_id().get_u32_value().unwrap()),
            name: "sample-stream".to_string(),
        })
        .await
    {
        Ok(_) => (),
        Err(err) => return Err(Box::from(err)),
    }

    match client
        .create_topic(&CreateTopic {
            stream_id: iggy_config.stream_id(),
            topic_id: Some(iggy_config.stream_id().get_u32_value().unwrap()),
            partitions_count: 1,
            name: "sample-topic".to_string(),
            message_expiry: None,
            max_topic_size: None,
            replication_factor: 1,
        })
        .await
    {
        Ok(_) => (),
        Err(err) => return Err(Box::from(err)),
    }

    Ok(())
}

/// Shuts down the connection to the Iggy server and logs out the user.
///
/// # Arguments
///
/// * `client` - A reference to an `IggyClient` instance, which is used to connect to the Iggy server and perform operations on it.
///
/// # Returns
///
/// A `Result` type, which can either be an `Ok` variant containing a unit value (`()`) or an `Err` variant containing a boxed dynamic error.
/// The `Ok` variant indicates that the user was successfully logged out and the connection was closed, while the `Err` variant indicates that there was an error while logging out or closing the connection.
///
///
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

/// Creates a new user on the Iggy server.
///
/// # Arguments
///
/// * `client` - A reference to an `IggyClient` instance, which is used to connect to the Iggy server and perform operations on it.
/// * `user` - A reference to an `IggyUser` instance, which contains the information about the user that is being created.
///
/// # Returns
///
/// A `Result` type, which can either be an `Ok` variant containing a unit value (`()`) or an `Err` variant containing a boxed dynamic error.
/// The `Ok` variant indicates that the user was successfully created, while the `Err` variant indicates that the user already exists.
///
///
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

/// Creates a new token for the logged in user.
///
/// This method takes a name and creates a new token with that name for the logged in user.
///
/// # Parameters
///
/// * `client` - The IggyClient instance
/// * `token_name` - The name of the token to create
///
/// # Returns
/// * A string containing the new token
///
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

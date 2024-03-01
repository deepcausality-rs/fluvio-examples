use common::prelude::{IggyConfig, IggyUser};
use iggy::client::{Client, UserClient};
use iggy::client::{PersonalAccessTokenClient, StreamClient, TopicClient};
use iggy::clients::client::{IggyClient, IggyClientBuilder};
use iggy::error::IggyError;
use iggy::models::user_status::UserStatus;
use iggy::personal_access_tokens::create_personal_access_token::CreatePersonalAccessToken;
use iggy::streams::create_stream::CreateStream;
use iggy::streams::delete_stream::DeleteStream;
use iggy::topics::create_topic::CreateTopic;
use iggy::topics::delete_topic::DeleteTopic;
use iggy::users::create_user::CreateUser;
use iggy::users::login_user::LoginUser;
use iggy::users::logout_user::LogoutUser;
use std::error::Error;

pub async fn get_consumer(
    iggy_config: &IggyConfig,
    user: &IggyUser,
) -> Result<IggyClient, IggyError> {
    let tcp_server_addr = iggy_config.tcp_server_addr();

    let consumer = get_iggy_client(tcp_server_addr)
        .await
        .expect("Failed to build iggy client");

    init_consumer(&consumer, &user)
        .await
        .expect("Failed to initialize iggy");

    Ok(consumer)
}

pub async fn get_producer(
    iggy_config: &IggyConfig,
    user: &IggyUser,
) -> Result<IggyClient, IggyError> {
    let tcp_server_addr = iggy_config.tcp_server_addr();

    let producer = get_iggy_client(tcp_server_addr)
        .await
        .expect("Failed to create consumer client");

    init_producer(&producer, &iggy_config, &user)
        .await
        .expect("Failed to initialize iggy");

    Ok(producer)
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
pub async fn get_iggy_client(tcp_server_addr: String) -> Result<IggyClient, IggyError> {
    IggyClientBuilder::new()
        .with_tcp()
        .with_server_address(tcp_server_addr)
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
pub async fn init_consumer(client: &IggyClient, user: &IggyUser) -> Result<(), Box<dyn Error>> {
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
        Ok(_) => (),
        Err(err) => return Err(Box::from(err)),
    }

    Ok(())
}

/// Initializes a producer by creating a stream and a topic in the Iggy platform.
///
/// This asynchronous function takes an `IggyClient` and `IggyConfig` references to create a stream
/// and a topic with the specified configurations. It ensures that the necessary infrastructure
/// is set up in Iggy for producing messages.
///
/// # Arguments
///
/// * `client` - A reference to the `IggyClient` used to interact with the Iggy platform.
/// * `iggy_config` - A reference to the `IggyConfig` which contains the configuration for the stream and topic.
///
/// # Returns
///
/// A `Result` type that is either:
/// - `Ok(())` - Indicates successful creation of both the stream and topic.
/// - `Err(Box<dyn Error>)` - An error occurred during the creation process. The error is boxed to allow for any type of `Error` trait object.
///
///
/// # Errors
///
/// This function will return an error if the stream or topic creation fails.
/// The error will be a boxed `Error` trait object,
/// which can represent any error that implements the `Error` trait.
///
pub async fn init_producer(
    client: &IggyClient,
    iggy_config: &IggyConfig,
    user: &IggyUser,
) -> Result<(), Box<dyn Error>> {
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
        Ok(_) => (),
        Err(err) => return Err(Box::from(err)),
    }

    match client
        .create_stream(&CreateStream {
            stream_id: Some(iggy_config.stream_id().get_u32_value().unwrap()),
            name: iggy_config.stream_name().to_string(),
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
            name: iggy_config.topic_name().to_string(),
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
pub async fn shutdown(
    client: &IggyClient,
    iggy_config: &IggyConfig,
    delete: bool,
) -> Result<(), Box<dyn Error>> {
    if delete {
        match client
            .delete_topic(&DeleteTopic {
                stream_id: iggy_config.stream_id(),
                topic_id: iggy_config.topic_id(),
            })
            .await
        {
            Ok(_) => (),
            Err(err) => return Err(Box::from(err)),
        }

        match client
            .delete_stream(&DeleteStream {
                stream_id: iggy_config.stream_id(),
            })
            .await
        {
            Ok(_) => (),
            Err(err) => return Err(Box::from(err)),
        }
    }

    match client.logout_user(&LogoutUser {}).await {
        Ok(_) => println!("* Iggy user logged out."),
        Err(_) => println!("* Iggy user was already logged out."),
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

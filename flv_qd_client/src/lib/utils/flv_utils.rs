use common::prelude::MessageClientConfig;
use fluvio::metadata::objects::CommonCreateRequest;
use fluvio::metadata::topic::TopicSpec;
use fluvio::{Fluvio, FluvioAdmin, TopicProducer};
use std::error::Error;

/// Creates the Fluvio topics needed for a client connection.
///
/// # Arguments
///
/// * `admin` - A reference to the Fluvio admin client to use for topic creation.
///
/// * `client_config` - A reference to the client configuration.
///
/// # Returns
///
/// Returns a `Result` with `()` on success, or an `Error` on failure.
///
/// This function creates the following topics using the Fluvio admin client:
///
/// 1. `client_id-control` - For receiving control messages from the gateway.
///    This is created from `client_config.control_channel()`.
///
/// 2. `client_id-data` - For receiving data messages from the gateway.
///    This is created from `client_config.data_channel()`.
///
/// 3. `client_id-error` - For receiving error messages from the gateway.
///    This is created from `client_config.error_channel()`.
///
pub(crate) async fn create_topics(
    admin: &FluvioAdmin,
    client_config: &MessageClientConfig,
) -> Result<(), Box<dyn Error>> {
    // Create client topics:
    // 1. client_id-control - For receiving control messages from the gateway.
    let control_topic = &client_config.control_channel();
    create_topic(&admin, control_topic)
        .await
        .expect("Failed to create control topic");

    // 2. client_id-data - For receiving data messages from the gateway.
    let data_topic = &client_config.data_channel();
    create_topic(&admin, data_topic)
        .await
        .expect("Failed to create data topic");

    // 3. client_id-error - For receiving error messages from the gateway.
    let err_topic = &client_config.error_channel();
    create_topic(&admin, err_topic)
        .await
        .expect("Failed to create data topic");

    Ok(())
}

/// Creates a new topic with the given name using the provided Fluvio admin client.
///
/// # Arguments
///
/// * `admin` - A reference to the Fluvio admin client to use.
/// * `topic_name` - The name for the new topic.
///
/// # Returns
///
/// Returns a Result with `()` on success, or an error on failure.
///
async fn create_topic(admin: &FluvioAdmin, topic_name: &str) -> Result<(), Box<dyn Error>> {
    // Define a new topic
    let name = topic_name.to_string();
    let dry_run = false;
    let common_request = CommonCreateRequest {
        name,
        dry_run,
        ..Default::default()
    };

    // Ok, specify the topic config
    let topic_specs = TopicSpec::new_computed(1, 1, None);

    // Create the topic
    admin
        .create_with_config(common_request, topic_specs)
        .await
        .expect("Failed to create topic");

    Ok(())
}

/// Deletes the Fluvio topics associated with a client configuration.
///
/// # Arguments
///
/// * `admin` - A reference to the Fluvio admin client to use for deletion.
///
/// * `client_config` - The client configuration containing the topic names.
///
/// # Returns
///
/// Returns a `Result` with `()` on success, or an `Error` on failure.
///
/// This deletes the following topics using the Fluvio admin client:
///
/// 1. `client_id-control` - The control topic from `client_config.control_channel()`.
///
/// 2. `client_id-data` - The data topic from `client_config.data_channel()`.
///
/// 3. `client_id-error` - The error topic from `client_config.error_channel()`.
///
pub(crate) async fn delete_topics(
    admin: &FluvioAdmin,
    client_config: &MessageClientConfig,
) -> Result<(), Box<dyn Error>> {
    // 1. client_id-control
    let control_topic = &client_config.control_channel();
    delete_topic(&admin, control_topic)
        .await
        .expect("Failed to delete topic");

    // 2. client_id-data
    let data_topic = &client_config.data_channel();
    delete_topic(&admin, data_topic)
        .await
        .expect("Failed to delete topic");

    // 3. client_id-error
    let err_topic = &client_config.error_channel();
    delete_topic(&admin, err_topic)
        .await
        .expect("Failed to delete topic");

    Ok(())
}

/// Deletes the Fluvio topic with the given name using the provided admin client.
///
/// # Arguments
///
/// * `admin` - A reference to the Fluvio admin client to use.
/// * `topic_name` - The name of the topic to delete.
///
/// # Returns
///
/// Returns a Result with `()` on success, or an error on failure.
///
async fn delete_topic(admin: &FluvioAdmin, topic_name: &str) -> Result<(), Box<dyn Error>> {
    admin
        .delete::<TopicSpec>(topic_name.to_string())
        .await
        .expect("Failed to delete topic");

    Ok(())
}

/// Creates a Fluvio TopicProducer for the given topic name.
///
/// # Arguments
///
/// * `topic` - The name of the topic to create the producer for.
///
/// # Returns
///
/// Returns a `Result` with the `TopicProducer` on success, or an `Error` on failure.
///
/// This connects to Fluvio using the default configuration, and then creates a
/// TopicProducer for sending messages to the specified topic.
///
pub(crate) async fn get_producer(topic: &str) -> Result<TopicProducer, Box<dyn Error>> {
    let fluvio = Fluvio::connect()
        .await
        .expect("Failed to connect to fluvio admin ");

    let producer = fluvio
        .topic_producer(topic)
        .await
        .expect("Failed to create a fluvio producer");

    Ok(producer)
}

/// Connects to the Fluvio admin API and returns a FluvioAdmin client.
///
/// # Returns
///
/// Returns a `Result` with the `FluvioAdmin` client on success, or an `Error` on failure.
///
/// This connects to the Fluvio admin API using the default configuration.
///
pub(crate) async fn get_admin() -> Result<FluvioAdmin, Box<dyn Error>> {
    let admin = FluvioAdmin::connect()
        .await
        .expect("Failed to connect to fluvio admin");

    Ok(admin)
}

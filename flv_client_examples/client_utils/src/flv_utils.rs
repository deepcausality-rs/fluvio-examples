use fluvio::{Fluvio, FluvioAdmin, PartitionConsumer, RecordKey, TopicProducer};
use std::error::Error;
use fluvio::metadata::objects::{CommonCreateRequest, DeleteRequest};
use fluvio::metadata::topic::config::TopicConfig;
use fluvio::metadata::topic::TopicSpec;

/// Sends a message to the given Fluvio topic producer.
///
/// # Arguments
///
/// * `producer` - The Fluvio topic producer to send the message to.
/// * `buffer` - The message payload as a byte buffer.
///
/// # Returns
///
/// Returns a Result with `()` on success, or an error on failure.
///
pub async fn send_message(producer: &TopicProducer, buffer: Vec<u8>) -> Result<(), Box<dyn Error>> {
    producer
        .send(RecordKey::NULL, buffer)
        .await
        .expect("Failed to send Done!");

    producer.flush().await.expect("Failed to flush");

    Ok(())
}

/// Creates a new topic with the given name on the Fluvio cluster.
///
/// # Arguments
///
/// * `topic_name` - The name for the new topic to create.
///
/// # Returns
///
/// Returns a Result with `()` on success, or an error on failure.
///
pub async fn create_topic(topic_name: &str) -> Result<(), Box<dyn Error>> {

    // Instantiate the admin client
    let admin = FluvioAdmin::connect()
        .await
        .expect("Failed to connect to Fluvio admin API");

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
    admin.create_with_config(common_request, topic_specs)
        .await
        .expect("Failed to create topic");

    Ok(())
}

pub async fn delete_topic(topic_name: &str) -> Result<(), Box<dyn Error>> {

    // Instantiate the admin client
    let admin = FluvioAdmin::connect()
        .await
        .expect("Failed to connect to Fluvio admin API");

    let name = topic_name.to_string();


    let topic_config = TopicConfig::default();

    let topic_specs = TopicSpec::from(topic_config);

    let delete_topic_request: DeleteRequest<TopicSpec> = DeleteRequest::new(name);


    // Delete the topic
    admin.delete(delete_topic_request).await.expect("Failed to delete topic");
    //
    //^^^^^^ cannot infer type for type parameter `S` declared on the method `delete`

    Ok(())
}

/// Creates a new Fluvio topic producer for the given topic name.
///
/// # Arguments
///
/// * `topic` - The name of the Fluvio topic to produce to.
///
/// # Returns
///
/// Returns a `TopicProducer` that can be used to produce records to the topic.
///
pub async fn get_producer(topic: &str) -> TopicProducer {
    let fluvio = Fluvio::connect().await.unwrap();

    fluvio
        .topic_producer(topic)
        .await
        .expect("Failed to create a producer")
}

/// Creates a new partition consumer for the given Fluvio topic.
///
/// # Arguments
///
/// * `topic` - The name of the Fluvio topic to consume from.
///
/// # Returns
///
/// Returns a `PartitionConsumer` for consuming records from the topic.
///
pub async fn get_consumer(topic: &str) -> PartitionConsumer {
    fluvio::consumer(topic, 0)
        .await
        .expect("Failed to create a consumer")
}

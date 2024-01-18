use fluvio::metadata::objects::CommonCreateRequest;
use fluvio::metadata::topic::TopicSpec;
use fluvio::{Fluvio, FluvioAdmin, PartitionConsumer, RecordKey, TopicProducer};
use std::error::Error;

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
pub async fn create_topic(admin: &FluvioAdmin, topic_name: &str) -> Result<(), Box<dyn Error>> {
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
pub async fn delete_topic(admin: &FluvioAdmin, topic_name: &str) -> Result<(), Box<dyn Error>> {
    admin
        .delete::<TopicSpec>(topic_name.to_string())
        .await
        .expect("Failed to delete topic");

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

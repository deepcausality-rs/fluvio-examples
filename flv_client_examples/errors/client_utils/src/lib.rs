use fluvio::{Fluvio, PartitionConsumer, RecordKey, TopicProducer};
use std::error::Error;

pub async fn send_message(producer: &TopicProducer, buffer: Vec<u8>) -> Result<(), Box<dyn Error>> {
    producer
        .send(RecordKey::NULL, buffer)
        .await
        .expect("Failed to send Done!");

    producer.flush().await.expect("Failed to flush");

    Ok(())
}

pub async fn get_producer(topic: &str) -> TopicProducer {
    let fluvio = Fluvio::connect().await.unwrap();

    fluvio
        .topic_producer(topic)
        .await
        .expect("Failed to create a producer")
}

pub async fn get_consumer(topic: &str) -> PartitionConsumer {
    fluvio::consumer(topic, 0)
        .await
        .expect("Failed to create a consumer")
}

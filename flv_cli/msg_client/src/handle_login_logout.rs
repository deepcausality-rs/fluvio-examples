use crate::utils;
use sbe_messages::prelude::{ClientLoginMessage, ClientLogoutMessage};
use std::error::Error;
use tokio::time::{sleep, Duration};

pub async fn handle() -> Result<(), Box<dyn Error>> {
    let produce_handle = tokio::spawn(produce());

    match tokio::try_join!(produce_handle) {
        Ok(_) => {}
        Err(e) => {
            println!(
                " Failed to start produce_handle and consume_handle: {:?}",
                e
            );
        }
    }

    Ok(())
}

async fn produce() -> Result<(), Box<dyn Error + Send>> {
    let service_topic = "qdgw-control";
    let producer = utils::get_producer(service_topic).await;

    let client_id = 100;
    let message = ClientLoginMessage::new(client_id);

    let enc = message.encode();
    assert!(enc.is_ok());

    println!("Send login message: {:?}", message);
    let (_, buffer) = enc.unwrap();
    utils::send_message(&producer, buffer)
        .await
        .expect("Failed to send message!");
    println!("Send!");
    println!();

    sleep(Duration::from_millis(250)).await;

    let message = ClientLogoutMessage::new(client_id);

    let enc = message.encode();
    assert!(enc.is_ok());

    println!("Send logout message: {:?}", message);
    let (_, buffer) = enc.unwrap();
    utils::send_message(&producer, buffer)
        .await
        .expect("Failed to send message!");
    println!("Send!");
    println!();

    Ok(())
}

use std::error::Error;
use fluvio::{FluvioAdmin, TopicProducer};
use common::prelude::MessageClientConfig;

mod utils;
mod login;

pub struct QDClient {
    admin: FluvioAdmin,
    client_config: MessageClientConfig,
    consumer: Result<(), Box<dyn Error + Send>>,
    producer: TopicProducer,
}

impl QDClient {

}
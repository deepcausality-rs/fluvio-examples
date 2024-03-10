use std::collections::HashMap;

use iggy::clients::client::IggyClient;
use iggy::messages::poll_messages::{PollMessages, PollingStrategy};

use common::prelude::IggyConfig;
use db_query_manager::QueryDBManager;
use symbol_manager::SymbolManager;

mod handle;
mod run;
mod utils;

// tokio RwLock implements sync + send and works well with tokio async
// https://stackoverflow.com/questions/67277282/async-function-the-trait-stdmarkersend-is-not-implemented-for-stdsync
type Guarded<T> = std::sync::Arc<tokio::sync::RwLock<T>>;

pub struct Server {
    consumer: IggyClient,
    producer: IggyClient,
    iggy_config: IggyConfig,
    poll_command: PollMessages,
    query_manager: Guarded<QueryDBManager>,
    symbol_manager: Guarded<SymbolManager>,
    client_configs: Guarded<HashMap<u16, IggyConfig>>,
    client_producers: Guarded<HashMap<u16, IggyClient>>,
}

impl Server {
    pub async fn new(
        iggy_config: IggyConfig,
        query_manager: Guarded<QueryDBManager>,
        symbol_manager: Guarded<SymbolManager>,
    ) -> Self {
        // Create an iggy client and initialize it as consumer
        let consumer = iggy_utils::get_consumer(&iggy_config)
            .await
            .expect("Failed to create consumer client");

        // Create an iggy client and initialize it as producer
        let producer = iggy_utils::get_producer(&iggy_config)
            .await
            .expect("Failed to create producer client");

        // Preconfigure the poll message command for the consumer client
        let poll_command = PollMessages {
            consumer: Default::default(),
            stream_id: iggy_config.stream_id(),
            topic_id: iggy_config.topic_id(),
            partition_id: Option::from(iggy_config.partition_id()),
            strategy: PollingStrategy::last(),
            count: iggy_config.messages_per_batch(),
            auto_commit: iggy_config.auto_commit(),
        };

        // Create a new HashMap to store configs for each client
        let client_configs = std::sync::Arc::new(tokio::sync::RwLock::new(HashMap::new()));

        // Create a new HashMap to store data producers for each client
        let client_producers = std::sync::Arc::new(tokio::sync::RwLock::new(HashMap::new()));

        Self {
            consumer,
            producer,
            iggy_config,
            poll_command,
            query_manager,
            symbol_manager,
            client_configs,
            client_producers,
        }
    }
}

impl Server {
    pub fn client_configs(&self) -> &Guarded<HashMap<u16, IggyConfig>> {
        &self.client_configs
    }
    pub fn client_producers(&self) -> &Guarded<HashMap<u16, IggyClient>> {
        &self.client_producers
    }
    pub fn consumer(&self) -> &IggyClient {
        &self.consumer
    }
    pub fn iggy_config(&self) -> &IggyConfig {
        &self.iggy_config
    }
    pub fn poll_command(&self) -> &PollMessages {
        &self.poll_command
    }
    pub fn producer(&self) -> &IggyClient {
        &self.producer
    }
    pub fn query_manager(&self) -> &Guarded<QueryDBManager> {
        &self.query_manager
    }
    pub fn symbol_manager(&self) -> &Guarded<SymbolManager> {
        &self.symbol_manager
    }
}

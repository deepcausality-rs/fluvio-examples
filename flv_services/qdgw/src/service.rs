use client_manager::ClientManager;
use common::prelude::IggyConfig;
use db_query_manager::QueryDBManager;
use fluvio::TopicProducer;
use iggy::messages::poll_messages::{PollMessages, PollingStrategy};
use std::collections::HashMap;
use std::sync::Arc;
use symbol_manager::SymbolManager;
// Future RwLock implements sync + send and works well with tokio async
// https://stackoverflow.com/questions/67277282/async-function-the-trait-stdmarkersend-is-not-implemented-for-stdsync
use tokio::sync::RwLock;

pub struct Server {
    // Add iggy producer to the server struct
    // Add iggy consumer to the server struct
    // iggy_config: IggyConfig,
    poll_command: PollMessages,
   client_manager: Arc<RwLock<ClientManager>>,
    query_manager: Arc<RwLock<QueryDBManager>>,
    symbol_manager: Arc<RwLock<SymbolManager>>,
    // Store a data producer for each client on login to send data back to the client
    client_data_producers: Arc<RwLock<HashMap<u16, TopicProducer>>>,
}

impl Server {
    pub fn new(
        iggy_config: IggyConfig,
        client_manager: Arc<RwLock<ClientManager>>,
        query_manager: Arc<RwLock<QueryDBManager>>,
        symbol_manager: Arc<RwLock<SymbolManager>>,
    ) -> Self {
        // Preconfigure the poll message command
        let poll_command = PollMessages {
            consumer: Default::default(),
            stream_id: iggy_config.stream_id(),
            topic_id: iggy_config.topic_id(),
            partition_id: iggy_config.partition_id(),
            strategy: PollingStrategy::last(),
            count: iggy_config.messages_per_batch(),
            auto_commit: iggy_config.auto_commit(),
        };

        // Create a new HashMap to store data producers for each client
        let client_data_producers = Arc::new(RwLock::new(HashMap::new()));

        Self {
            // iggy_config,
            poll_command,
            client_manager,
            query_manager,
            symbol_manager,
            client_data_producers,
        }
    }
}

impl Server {
    pub fn client_manager(&self) -> &Arc<RwLock<ClientManager>> {
        &self.client_manager
    }
    pub fn poll_command(&self) -> &PollMessages {
        &self.poll_command
    }
    pub fn query_manager(&self) -> &Arc<RwLock<QueryDBManager>> {
        &self.query_manager
    }
    pub fn symbol_manager(&self) -> &Arc<RwLock<SymbolManager>> {
        &self.symbol_manager
    }
    pub fn client_data_producers(&self) -> &Arc<RwLock<HashMap<u16, TopicProducer>>> {
        &self.client_data_producers
    }
}

use client_manager::ClientManager;
use common::prelude::{IggyConfig, IggyUser, MessageProcessingError};
use db_query_manager::QueryDBManager;
use fluvio::{TopicProducer};
use futures::StreamExt;
use iggy_utils;
use std::collections::HashMap;
use std::future::Future;
use std::sync::Arc;
use iggy::client::MessageClient;
use iggy::messages::poll_messages::{PollingStrategy, PollMessages};
use symbol_manager::SymbolManager;
use tokio::sync::RwLock;
use tokio::{pin, select};

pub struct Server {
    channel_topic: String,
    iggy_config: IggyConfig,
    // Future RwLock implements sync + send and works well
    // with tokio async https://stackoverflow.com/questions/67277282/async-function-the-trait-stdmarkersend-is-not-implemented-for-stdsync
    pub(crate) client_manager: Arc<RwLock<ClientManager>>,
    pub(crate) query_manager: Arc<RwLock<QueryDBManager>>,
    pub(crate) symbol_manager: Arc<RwLock<SymbolManager>>,
    // Store a data producer for each client on login to send data back to the client
    pub(crate) client_data_producers: Arc<RwLock<HashMap<u16, TopicProducer>>>,
}

impl Server {
    /// Creates a new Server instance.
    ///
    /// # Parameters
    ///
    /// * `channel_topic` - The Fluvio topic to subscribe to for incoming messages
    /// * `client_manager` - Manager for tracking connected clients
    /// * `query_manager` - Manager for queries to the database
    /// * `symbol_manager` - Manager for symbol metadata
    ///
    /// # Returns
    ///
    /// A new Server instance
    ///
    pub fn new(
        channel_topic: String,
        iggy_config: IggyConfig,
        client_manager: Arc<RwLock<ClientManager>>,
        query_manager: Arc<RwLock<QueryDBManager>>,
        symbol_manager: Arc<RwLock<SymbolManager>>,
    ) -> Self {
        // Create a new HashMap to store data producers for each client
        let client_data_producers = Arc::new(RwLock::new(HashMap::new()));

        Self {
            channel_topic,
            iggy_config,
            client_manager,
            query_manager,
            symbol_manager,
            client_data_producers,
        }
    }
}

impl Server {
    /// Runs the server, listening for signals and incoming messages.
    ///
    /// # Parameters
    ///
    /// * `self` - The Server instance
    /// * `signal` - A future that resolves when a shutdown signal is received
    ///
    /// # Functionality
    ///
    /// - Creates a consumer for the channel topic to receive messages
    /// - Creates a stream of messages from the consumer
    /// - Enters a loop selecting on the shutdown signal future and stream:
    ///   - If signal arrives, breaks the loop to shutdown
    ///   - If stream has a message, calls handle_record() to process it
    ///
    /// # Returns
    /// * Ok on success,
    /// * Err on any stream or message processing error
    ///
    pub async fn run(
        self,
        signal: impl Future<Output = ()> + Send + 'static,
    ) -> Result<(), MessageProcessingError> {
        // When call .await on a &mut _ reference, pin the future. https://docs.rs/tokio/latest/tokio/macro.pin.html#examples
        let signal_future = signal;
        pin!(signal_future);

        let client = iggy_utils::get_iggy_client()
            .await
            .expect("Failed to create client");

        let user = IggyUser::default();
        iggy_utils::init(&client, &user)
            .await
            .expect("Failed to initialize iggy");

        let command = PollMessages {
            consumer: Default::default(),
            stream_id: self.iggy_config.stream_id(),
            topic_id: self.iggy_config.topic_id(),
            partition_id: self.iggy_config.partition_id(),
            strategy: PollingStrategy::last(),
            count: self.iggy_config.messages_per_batch(),
            auto_commit: self.iggy_config.auto_commit(),
        };

        loop {
            select! {
                    _ = &mut signal_future => {
                         break;
                    }

                polled_messages = client.poll_messages(&command).await => {
                    match polled_messages {
                        Ok(polled_messages) => {
                            for polled_message in polled_messages.messages {
                                self.handle_record(polled_message)
                                   .await.expect("Failed to process message");
                            }
                        },
                        Err(e) => {
                            println!("[QDGW/Service:run]: Error polling messages: {}", e);
                            break;
                        }
                    }
                } // end match polled messages
            } // end select

        } // end loop

        // Shutdown iggy
        iggy_utils::shutdown(&client)
            .await
            .expect("Failed to shutdown iggy");

        Ok(())
    }
}

use client_manager::ClientManager;
use common::prelude::MessageProcessingError;
use db_query_manager::QueryDBManager;
use fluvio::{Offset, TopicProducer};
use futures::lock::Mutex;
use futures::StreamExt;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::time;
use symbol_manager::SymbolManager;
use tokio::time::sleep;
use tokio::{pin, select};

// fixed retry for simplicity, but exponetial backoff crates are available
const RETRY: time::Duration = time::Duration::from_secs(5);

pub struct Server {
    channel_topic: String,
    // Future Mutex implements sync + send and works well
    // with tokio async https://stackoverflow.com/questions/67277282/async-function-the-trait-stdmarkersend-is-not-implemented-for-stdsync
    pub(crate) client_manager: Arc<Mutex<ClientManager>>,
    pub(crate) query_manager: Arc<Mutex<QueryDBManager>>,
    pub(crate) symbol_manager: Arc<Mutex<SymbolManager>>,
    // Store a data producer for each client on login to send data back to the client
    pub(crate) client_data_producers: Arc<Mutex<HashMap<u16, TopicProducer>>>,
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
        client_manager: Arc<Mutex<ClientManager>>,
        query_manager: Arc<Mutex<QueryDBManager>>,
        symbol_manager: Arc<Mutex<SymbolManager>>,
    ) -> Self {
        // Create a new HashMap to store data producers for each client
        let client_data_producers = Arc::new(Mutex::new(HashMap::new()));

        Self {
            channel_topic,
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
        //
        // When call .await on a &mut _ reference, pin the future. https://docs.rs/tokio/latest/tokio/macro.pin.html#examples
        let signal_future = signal;
        pin!(signal_future);

        // Handle reconnecting to the cluster if the connection fails.
        loop {
            // Connect to the Fluvio cluster.
            let Ok(client) = fluvio::consumer(&self.channel_topic, 0).await else {
                println!("[QDGW/Service:run]: Could not connect to Fluvio cluster, retrying");
                sleep(RETRY).await;
                continue;
            };

            // Creates a stream of messages from the topic.
            let Ok(mut stream) = client.stream(Offset::end()).await else {
                println!("[QDGW/Service:run]: Failed to create stream, retrying");
                sleep(RETRY).await;
                continue;
            };

            loop {
                select! {
                        record = stream.next() => {
                            if let Some(res) = record {
                                         match res {
                                             Ok(record) => {
                                                 match self.handle_record(&record).await{
                                                Ok(()) => {},
                                                Err(e) => {
                                                    return Err(e);
                                                }
                                            }
                                         },
                                            Err(e) =>{
                                                println!("[QDGW/Service:run]: Reconnecting stream");
                                                sleep(RETRY).await;
                                                // Exit inner loop and try to reconnect stream
                                                break;
                                            }
                                     }
                                 }// end match record
                    }// end stream.next()
                } // end select
            } // End inner loop
        } // end outer loop

        // currently unreachable, because the signal handler is not yet implemented
        Ok(())
    }
}

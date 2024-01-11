use client_manager::ClientManager;
use common::prelude::MessageProcessingError;
use db_query_manager::QueryDBManager;
use fluvio::{Offset, PartitionConsumer};
use futures::lock::Mutex;
use futures::StreamExt;
use std::future::Future;
use std::sync::Arc;
use symbol_manager::SymbolManager;
use tokio::{pin, select};

// Future Mutex implements sync + send and works well with tokio async
// https://stackoverflow.com/questions/67277282/async-function-the-trait-stdmarkersend-is-not-implemented-for-stdsync

pub struct Server {
    consumer: PartitionConsumer,
    pub(crate) client_manager: Arc<Mutex<ClientManager>>,
    pub(crate) query_manager: Arc<Mutex<QueryDBManager>>,
    pub(crate) symbol_manager: Arc<Mutex<SymbolManager>>,
}

impl Server {
    pub fn new(
        consumer: PartitionConsumer,
        client_manager: Arc<Mutex<ClientManager>>,
        query_manager: Arc<Mutex<QueryDBManager>>,
        symbol_manager: Arc<Mutex<SymbolManager>>,
    ) -> Self {
        Self {
            consumer,
            client_manager,
            query_manager,
            symbol_manager,
        }
    }
}

impl Server {
    pub async fn run(
        self,
        signal: impl Future<Output = ()> + Send + 'static,
    ) -> Result<(), MessageProcessingError> {
        // When call .await on a &mut _ reference, pin the future. https://docs.rs/tokio/latest/tokio/macro.pin.html#examples
        let signal_future = signal;
        pin!(signal_future);

        // Creates a stream of messages from the topic.
        let mut stream = self
            .consumer
            .stream(Offset::end())
            .await
            .expect("[QDGW/Service:run]: Failed to create a stream");

        loop {
            select! {
                    _ = &mut signal_future => {
                         break;
                    }

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
                                             return Err(MessageProcessingError(e.to_string()));
                                         }
                                 }
                             }
                }// end stream.next()
            } // end select
        } // end loop

        Ok(())
    }
}

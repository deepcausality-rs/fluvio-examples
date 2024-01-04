use client_manager::ClientManager;
use common::prelude::MessageProcessingError;
use fluvio::dataplane::record::ConsumerRecord;
use fluvio::{Offset, PartitionConsumer};
use futures::StreamExt;
use sbe_messages::prelude::{
    ClientLoginMessage, ClientLogoutMessage, MessageType, StartDataMessage, StopAllDataMessage,
    StopDataMessage,
};
use std::future::Future;
use std::sync::{Arc, Mutex};
use tokio::{pin, select};

pub struct Server {
    consumer: PartitionConsumer,
    // qd_manager: QDManager,
    client_manager: Arc<Mutex<ClientManager>>,
}

impl Server {
    pub fn new(
        consumer: PartitionConsumer,
        // qd_manager: QDManager,
        client_manager: Arc<Mutex<ClientManager>>,
    ) -> Self {
        Self {
            consumer,
            // qd_manager,
            client_manager,
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

    async fn handle_record(&self, record: &ConsumerRecord) -> Result<(), MessageProcessingError> {
        let value = record.get_value().to_vec();
        let buffer = value.as_slice();
        let message_type = MessageType::from(buffer[2]);

        match message_type {
            MessageType::UnknownMessageType => Err(MessageProcessingError(
                "[QDGW/handle::handle_record]:  Fluvio consumer record contained an unknown message type."
                    .to_string(),
            )),

            MessageType::ClientLogin => {
                let client_login_msg = ClientLoginMessage::from(buffer);
                self.client_login(&self.client_manager, &client_login_msg).await
            }

            MessageType::ClientLogout => {
                let client_logout_msg = ClientLogoutMessage::from(buffer);
                self.client_logout(&self.client_manager, &client_logout_msg).await
            }

            MessageType::StartData => {
                let start_data_msg = StartDataMessage::from(buffer);
                let client_id = start_data_msg.client_id();

                let client_data_channel = match self.get_client_data_channel(&self.client_manager, *client_id).await {
                    Ok(channel) => channel,
                    Err(e) => {
                        // Send error message back to client instead of return
                        return Err(e);
                    }
                };

                self.start_data(&client_data_channel, &start_data_msg).await
            }
            MessageType::StopData => {
                let stop_data_msg = StopDataMessage::from(buffer);
                self.stop_date(&stop_data_msg).await
            }
            MessageType::StopAllData => {
                let stop_all_data_msg = StopAllDataMessage::from(buffer);
                self.stop_all_data(&stop_all_data_msg).await
            }
            _ => {
                Ok(())
            }
        }
    }

    async fn get_client_data_channel(
        &self,
        client_manager: &Arc<Mutex<ClientManager>>,
        client_id: u16,
    ) -> Result<String, MessageProcessingError> {
        let client_db = client_manager.lock().unwrap();
        match client_db.get_client_data_channel(client_id) {
            Ok(data_channel) => Ok(data_channel),
            Err(e) => Err(MessageProcessingError(e.to_string())),
        }
    }
}

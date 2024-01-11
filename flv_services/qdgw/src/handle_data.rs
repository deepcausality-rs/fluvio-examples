use crate::service::Server;
use common::prelude::MessageProcessingError;
use db_query_manager::QueryDBManager;
use fluvio::{Fluvio, RecordKey};
use futures::lock::Mutex;
use sbe_messages::prelude::{
    FirstTradeBar, LastTradeBar, SbeTradeBar, StartDataMessage, StopAllDataMessage, StopDataMessage,
};
use std::sync::Arc;

impl Server {
    pub(crate) async fn start_data(
        &self,
        query_manager: &Arc<Mutex<QueryDBManager>>,
        client_data_channel: &str,
        start_data_msg: &StartDataMessage,
    ) -> Result<(), MessageProcessingError> {
        // Remove debug print
        println!(
            "[QDGW/handle::start_date]: start_data: {:?} on channel : {:?}",
            start_data_msg, client_data_channel
        );

        let fluvio = Fluvio::connect().await.unwrap();

        let producer = fluvio
            .topic_producer(client_data_channel)
            .await
            .expect("Failed to create a producer");

        // Replace these fields with dynamic configuration
        let symbol_id = *start_data_msg.symbol_id();
        let trade_table = "kraken_ethaed";

        // Lock query manager
        let mut q_manager = query_manager.lock().await;

        // Get all bars
        let result = q_manager.get_all_trades(trade_table).await;

        // Handle error
        let bars = match result {
            Ok(bars) => bars,
            Err(err) => {
                println!("[QDGW/handle::start_date]: Error getting bars: {:?}", err);
                return Err(MessageProcessingError("Failed to get bars".into()));
            }
        };

        // Send first  bar message to inform the client that the data stream starts
        let first_bar = FirstTradeBar::new(symbol_id);
        let (_, buffer) = first_bar
            .encode()
            .expect("Failed to encode first trade bar");

        producer
            .send(RecordKey::NULL, buffer)
            .await
            .expect("Failed to send Done!");
        producer.flush().await.expect("Failed to flush");

        for bar in bars {
            let (_, buffer) =
                SbeTradeBar::encode_data_bar_message(bar).expect("Failed to encode trade bar");

            producer
                .send(RecordKey::NULL, buffer)
                .await
                .expect("Failed to send Done!");
        }

        // Send last bar message to inform the client that the data stream has ended
        let last_bar = LastTradeBar::new(symbol_id);
        let (_, buffer) = last_bar.encode().expect("Failed to encode last trade bar");

        producer
            .send(RecordKey::NULL, buffer)
            .await
            .expect("Failed to send Done!");
        producer.flush().await.expect("Failed to flush");

        Ok(())
    }

    pub(crate) async fn stop_date(
        &self,
        stop_data_msg: &StopDataMessage,
    ) -> Result<(), MessageProcessingError> {
        // Remove debug print
        println!("[QDGW/handle::stop_date]: stop_data: {:?}", stop_data_msg);

        Ok(())
    }

    pub(crate) async fn stop_all_data(
        &self,
        stop_all_data_msg: &StopAllDataMessage,
    ) -> Result<(), MessageProcessingError> {
        // Remove debug print
        println!(
            "[QDGW/handle::stop_all_data]: stop_all_data: {:?}",
            stop_all_data_msg
        );

        Ok(())
    }
}

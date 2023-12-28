use crate::service::Server;
use common::prelude::{MessageProcessingError};
use sbe_messages::prelude::{
    StartDataMessage, StopAllDataMessage, StopDataMessage,
};

impl Server {
    pub(crate) async fn start_data(
        &self,
        // qd_manager: &QDManager,
        client_data_channel: &str,
        start_data_msg: &StartDataMessage,
    ) -> Result<(), MessageProcessingError> {
        // Remove debug print
        println!("[QDGW/handle::start_date]: start_data: {:?} on channel : {:?}", start_data_msg, client_data_channel);

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

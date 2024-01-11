use crate::service::Server;
use common::prelude::MessageProcessingError;
use sbe_messages::prelude::StopDataMessage;

impl Server {
    pub(crate) async fn stop_date(
        &self,
        stop_data_msg: &StopDataMessage,
    ) -> Result<(), MessageProcessingError> {
        // Remove debug print
        println!("[QDGW/handle::stop_date]: stop_data: {:?}", stop_data_msg);

        Ok(())
    }
}

use autometrics::autometrics;
use crate::service::Server;
use common::prelude::MessageProcessingError;
use sbe_messages::prelude::StopDataMessage;

impl Server {
    #[autometrics]
    pub(crate) async fn handle_stop_date(
        &self,
        stop_data_msg: &StopDataMessage,
    ) -> Result<(), MessageProcessingError> {
        // Remove debug print
        println!("[QDGW/handle::stop_date]: stop_data: {:?}", stop_data_msg);

        Ok(())
    }
}

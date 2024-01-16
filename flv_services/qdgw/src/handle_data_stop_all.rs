use autometrics::autometrics;
use crate::service::Server;
use common::prelude::MessageProcessingError;
use sbe_messages::prelude::StopAllDataMessage;

impl Server {
    #[autometrics]
    pub(crate) async fn handle_stop_all_data(
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

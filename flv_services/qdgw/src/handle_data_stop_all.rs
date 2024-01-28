use crate::service::Server;
use autometrics::autometrics;
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
            "[QDGW/handle_stop_date]: stop_all_data: {:?}",
            stop_all_data_msg
        );

        println!("[QDGW/handle_stop_date]: NOT IMPLEMENTED",);

        Ok(())
    }
}

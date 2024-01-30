use crate::service::Server;
use common::prelude::{MessageProcessingError, TradeBar};
use sbe_messages::prelude::DataErrorType;

impl Server {
    /// Sends a stream of trade bar data to the client.
    ///
    /// Sends a first bar message, followed by the trade bars, and finally a last bar message.
    ///
    /// # Parameters
    ///
    /// * `client_id` - The client ID to send the data to
    /// * `first_bar` - Encoded bytes of the first bar message
    /// * `data_bars` - The trade bars to send
    /// * `last_bar` - Encoded bytes of the last bar message
    ///
    /// # Returns
    ///
    /// Returns a `Result` with `()` if successful, otherwise returns a
    /// `(DataErrorType, MessageProcessingError)` tuple on failure to:
    ///
    pub(crate) async fn start_trade_data(
        &self,
        client_id: u16,
        first_bar: Vec<u8>,
        data_bars: &[TradeBar],
        last_bar: Vec<u8>,
    ) -> Result<(), (DataErrorType, MessageProcessingError)> {
        // Send first  bar message to inform the client that the data stream starts
        match self.send_single_data(client_id, first_bar).await {
            Ok(_) => {}
            Err(e) => {
                return Err(e);
            }
        }

        // Send the trade bar data vector in bulk to the client
        match self.send_bulk_trade_data(client_id, data_bars).await {
            Ok(_) => {}
            Err(e) => {
                return Err(e);
            }
        }

        // Send the last bar message to inform the client that the data stream has ended
        match self.send_single_data(client_id, last_bar).await {
            Ok(_) => {}
            Err(e) => {
                return Err(e);
            }
        }

        Ok(())
    }
}

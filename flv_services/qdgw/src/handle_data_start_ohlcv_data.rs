use crate::service::Server;
use common::prelude::{MessageProcessingError, OHLCVBar};
use sbe_messages::prelude::DataErrorType;

impl Server {
    /// Sends a stream of OHLCV bar data to the client.
    ///
    /// Sends a first bar message, followed by the OHLCV bars, and finally a last bar message.
    ///
    /// # Parameters
    ///
    /// * `client_id` - The client ID to send the data to
    /// * `first_bar` - Encoded bytes of the first bar message
    /// * `data_bars` - The OHLCV bars to send
    /// * `last_bar` - Encoded bytes of the last bar message
    ///
    /// # Returns
    ///
    /// Returns a `Result` with `()` if successful, otherwise returns a
    /// `(DataErrorType, MessageProcessingError)` tuple on failure to:
    ///
    pub(crate) async fn start_ohlcv_data(
        &self,
        client_id: u16,
        first_bar: Vec<u8>,
        data_bars: &Vec<OHLCVBar>,
        last_bar: Vec<u8>,
    ) -> Result<(), (DataErrorType, MessageProcessingError)> {
        // Send first  bar message to inform the client that the data stream starts
        match self.send_single_data(client_id, first_bar).await {
            Ok(_) => {}
            Err(e) => {
                return Err(e);
            }
        }

        // Send the OHLCV bar data vector in bulk to the client
        match self.send_bulk_ohlcv_data(client_id, data_bars).await {
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

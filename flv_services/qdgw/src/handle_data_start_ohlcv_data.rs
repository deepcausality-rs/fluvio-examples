use crate::service::Server;
use common::prelude::{MessageProcessingError, OHLCVBar};
use sbe_messages::prelude::{DataErrorType, SbeOHLCVBar};

impl Server {
    /// Sends a stream of OHLCV bar data to the client.
    ///
    /// Sends a first bar message, followed by the OHLCV bars, and finally a last bar message.
    ///
    /// # Parameters
    ///
    /// * `client_data_channel` - The Fluvio topic name to send the data to
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
        match self.send_data(client_id, first_bar).await {
            Ok(_) => {}
            Err(e) => {
                return Err(e);
            }
        }

        // Send all trade bars to the client
        for bar in data_bars.to_vec() {
            // Encode bar message
            let (_, buffer) = match SbeOHLCVBar::encode_data_bar_message(bar) {
                Ok(enc) => enc,
                Err(e) => {
                    return Err((
                        DataErrorType::DataEncodingError,
                        MessageProcessingError(e.to_string()),
                    ));
                }
            };

            // Send bar message to client
            match self.send_data(client_id, buffer).await {
                Ok(_) => {}
                Err(e) => {
                    return Err(e);
                }
            }
        }

        // Send the last bar message to inform the client that the data stream has ended
        match self.send_data(client_id, last_bar).await {
            Ok(_) => {}
            Err(e) => {
                return Err(e);
            }
        }

        Ok(())
    }
}

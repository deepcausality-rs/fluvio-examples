use crate::service::Server;
use autometrics::autometrics;
use common::prelude::MessageProcessingError;
use sbe_messages::prelude::{ClientErrorType, DataErrorType, DataType, StartDataMessage};

impl Server {
    /// Handles a StartDataMessage from a client.
    ///
    /// Validates the request, gets the requested data, and sends it to the client.
    ///
    /// # Parameters
    ///
    /// * `start_data_msg` - The StartDataMessage from the client
    ///
    /// # Returns
    ///
    /// Returns a `Result` with `()` if successful, otherwise returns a
    /// `MessageProcessingError` on failure.
    ///
    /// # Errors
    ///
    /// Can fail with a `MessageProcessingError` if:
    ///
    /// - Fails to get the client control or data channel
    /// - Fails when the client is not logged in
    /// - Fails when an unknown data type is requested
    /// - Fails to get the first or last bar
    /// - Fails to get the requested data bars
    /// - Fails to send any error messages back to the client
    ///
    #[autometrics]
    pub(crate) async fn handle_start_data_message(
        &self,
        start_data_msg: &StartDataMessage,
    ) -> Result<(), MessageProcessingError> {
        // Remove debug print
        println!(
            "[QDGW/handle::handle_start_data_message]: {:?}",
            &start_data_msg
        );

        // Extract fields from message
        let client_id = *start_data_msg.client_id();
        let exchange_id = *start_data_msg.exchange_id() as u8;
        let symbol_id = *start_data_msg.symbol_id();
        let data_type = start_data_msg.data_type_id();

        // Check if the client is logged in
        let exists = self.check_client_login(client_id).await.expect(
            "[QDGW/handle::handle_start_data_message]: Failed to check if client is logged in",
        );

        // Send a ClientNotLoggedIn Error, if not logged in.
        if !exists {
            let client_error_type = ClientErrorType::ClientNotLoggedIn;
            match self.send_client_error(client_id, client_error_type).await {
                Ok(_) => {}
                Err(err) => {
                    println!("[QDGW/handle::handle_start_data_message]: Failed to send DataTableNotFound error: {}", err);
                }
            }

            return Err(MessageProcessingError("Client not logged in.".into()));
        }

        let trade_table = match self.get_trade_table_name(exchange_id).await {
            Ok(table) => table,
            Err(err) => {
                let data_err = DataErrorType::DataTableNotFound;
                match self.send_data_error(client_id, data_err).await {
                    Ok(_) => {}
                    Err(err) => {
                        println!("[QDGW/handle::handle_start_data_message]: Failed to send DataTableNotFound error: {}", err);
                    }
                }
                return Err(MessageProcessingError(format!(
                    "Failed to get data table: {}",
                    err
                )));
            }
        };

        //  Get first bar
        let first_bar = match self.encode_first_trade_bar(data_type, symbol_id).await {
            Ok(bar) => bar,
            Err((data_err, err)) => {
                match self.send_data_error(client_id, data_err).await {
                    Ok(_) => {}
                    Err(err) => {
                        println!(
                            "[QDGW/handle::handle_start_data_message]: Failed to get first bar: {}",
                            err
                        );
                    }
                }
                return Err(MessageProcessingError(format!(
                    "Failed to get first bar: {}",
                    err
                )));
            }
        };

        //  Get last bar
        let last_bar = match self.encode_last_trade_bar(data_type, symbol_id).await {
            Ok(bar) => bar,
            Err((data_err, err)) => {
                match self.send_data_error(client_id, data_err).await {
                    Ok(_) => {}
                    Err(err) => {
                        println!(
                            "[QDGW/handle::handle_start_data_message]: Failed to get last bar: {}",
                            err
                        );
                    }
                }
                return Err(MessageProcessingError(format!(
                    "Failed to get last bar: {}",
                    err
                )));
            }
        };

        match data_type {
            DataType::UnknownDataType => {
                let data_err = DataErrorType::DataTypeNotKnownError;
                match self.send_data_error(client_id, data_err).await {
                    Ok(_) => {}
                    Err(err) => {
                        println!(
                            "[QDGW/handle::handle_start_data_message]: Failed to get last bar: {}",
                            err
                        );
                    }
                }
            }
            DataType::TradeData => {
                // Get all trade bars
                let result = self.get_trade_bars(symbol_id, &trade_table).await;

                // Handle query error error
                let trade_bars = match result {
                    Ok(bars) => bars,
                    Err(err) => {
                        let data_err = DataErrorType::DataUnavailableError;
                        match self.send_data_error(client_id, data_err).await {
                            Ok(_) => {}
                            Err(err) => {
                                println!("[QDGW/handle::handle_start_data_message]: Failed to get last bar: {}", err);
                            }
                        }

                        return Err(MessageProcessingError(format!(
                            "Failed to get trade bars: {}",
                            err
                        )));
                    }
                };

                match self
                    .start_trade_data(client_id, first_bar, &trade_bars, last_bar)
                    .await
                {
                    Ok(_) => {}
                    Err((data_err, err)) => {
                        match self.send_data_error(client_id, data_err).await {
                            Ok(_) => {}
                            Err(err) => {
                                println!("[QDGW/handle::handle_start_data_message]: Failed to get last bar: {}", err);
                            }
                        }

                        return Err(MessageProcessingError(format!(
                            "Failed to get trade bars: {}",
                            err
                        )));
                    }
                }
            }
            DataType::OHLCVData => {
                let time_resolution = &start_data_msg.time_resolution();

                // Handle query error error
                let ohlcv_bars = match self
                    .get_ohlcv_bars(symbol_id, time_resolution, &trade_table)
                    .await
                {
                    Ok(bars) => bars,
                    Err(err) => {
                        let data_err = DataErrorType::DataUnavailableError;
                        match self.send_data_error(client_id, data_err).await {
                            Ok(_) => {}
                            Err(err) => {
                                println!("[QDGW/handle::handle_start_data_message]: Failed to get last bar: {}", err);
                            }
                        }

                        return Err(MessageProcessingError(format!(
                            "Failed to get trade bars: {}",
                            err
                        )));
                    }
                };

                match self
                    .start_ohlcv_data(client_id, first_bar, &ohlcv_bars, last_bar)
                    .await
                {
                    Ok(_) => {}
                    Err((data_err, err)) => {
                        match self.send_data_error(client_id, data_err).await {
                            Ok(_) => {}
                            Err(err) => {
                                println!("[QDGW/handle::handle_start_data_message]: Failed to get last bar: {}", err);
                            }
                        }

                        return Err(MessageProcessingError(format!(
                            "Failed to get trade bars: {}",
                            err
                        )));
                    }
                }
            }
        };

        Ok(())
    }
}

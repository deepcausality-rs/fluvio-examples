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
        // println!("[QDGW/handle::handle_start_data_message]");

        // println!("::handle_start_data_message]: Extract fields from message");
        let client_id = *start_data_msg.client_id();
        let exchange_id = *start_data_msg.exchange_id() as u16;
        let symbol_id = *start_data_msg.symbol_id();
        let data_type = start_data_msg.data_type_id();

        // println!("::handle_start_data_message]: Check if the client is already logged in");
        let exists = self.check_client_login(client_id).await.expect(
            "[QDGW/handle::handle_start_data_message]: Failed to check if client is logged in",
        );

        // Send a ClientNotLoggedIn Error, if not logged in.
        if !exists {
            // println!("[::handle_start_data_message]: Client is not logged in, return an ClientNotLoggedIn error to the client");
            let client_error_type = ClientErrorType::ClientNotLoggedIn;
            match self.send_client_error(client_id, client_error_type).await {
                Ok(_) => {}
                Err(err) => {
                    println!("[QDGW/handle::handle_start_data_message]: Failed to send ClientNotLoggedIn error: {}", err);
                }
            }

            return Err(MessageProcessingError("Client not logged in.".into()));
        }

        // println!("[::handle_start_data_message]: Client is logged in, proceed.");
        // println!("[::handle_start_data_message]: Get trade data table.");
        let trade_table = match self.get_trade_table_name(exchange_id, symbol_id).await {
            Ok(table) => table,
            Err(err) => {
                println!("[QDGW/handle::handle_start_data_message]: Failed to get Data Table For exchange error: {}", err);
                let data_err = DataErrorType::DataTableNotFound;
                match self.send_data_error(client_id, data_err).await {
                    Ok(_) => {}
                    Err(err) => {
                        println!("[QDGW/handle::handle_start_data_message]: Failed to send DataTableNotFound error: {}", err);
                    }
                }
                return Err(MessageProcessingError(format!(
                    "[QDGW/handle::handle_start_data_message]: Failed to get data table: {}",
                    err
                )));
            }
        };

        // println!("[::handle_start_data_message]: Get first bar.");
        let first_bar = match self.encode_first_bar(data_type, symbol_id).await {
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
                    "[QDGW/handle::handle_start_data_message]: Failed to get first bar: {}",
                    err
                )));
            }
        };

        // println!("[::handle_start_data_message]: Get last bar.");
        let last_bar = match self.encode_last_bar(data_type, symbol_id).await {
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
                    "[QDGW/handle::handle_start_data_message]: Failed to get last bar: {}",
                    err
                )));
            }
        };

        // println!("[::handle_start_data_message]: Get trade bars for data type.");
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
                // println!("[::handle_start_data_message]: Get all trade bars.");
                // println!("[::handle_start_data_message]: Symbol: {}, trade table: {}", symbol_id, trade_table);
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
                            "[QDGW/handle::handle_start_data_message]: Failed to get trade bars: {}",
                            err
                        )));
                    }
                };

                // println!("[::handle_start_data_message]: send all trade bars to the client.");
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
                            "[QDGW/handle::handle_start_data_message]: Failed to get trade bars: {}",
                            err
                        )));
                    }
                }
            }
            DataType::OHLCVData => {
                // println!("[::handle_start_data_message]: Get all OHLCV bars.");

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
                            "[QDGW/handle::handle_start_data_message]: Failed to get trade bars: {}",
                            err
                        )));
                    }
                };

                // println!("[::handle_start_data_message]: send all OHLCV bars to the client.");
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
                            "[QDGW/handle::handle_start_data_message]: Failed to get trade bars: {}",
                            err
                        )));
                    }
                }
            }
        };

        // println!("[handle_start_data_message]: Date send successfully to client: {}", client_id);
        Ok(())
    }
}

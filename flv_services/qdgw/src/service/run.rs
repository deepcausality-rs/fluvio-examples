use std::error::Error;
use std::future::Future;

use iggy::client::MessageClient;
use tokio::{pin, select};

use common::prelude::MessageProcessingError;

use crate::service::Server;

impl Server {
    /// Runs the server, listening for signals and incoming messages.
    ///
    /// This method will create a consumer for the channel topic to receive messages,
    /// create a stream of messages from the consumer, and enter a loop selecting on
    /// the shutdown signal future and stream.
    /// If the signal arrives, the loop will break and shutdown.
    /// If the stream has a message, the `handle_record()` method will be called to process it.
    ///
    /// # Parameters
    ///
    /// * `self` - The Server instance
    /// * `signal` - A future that resolves when a shutdown signal is received
    ///
    /// # Returns
    /// * Ok on success,
    /// * Err on any stream or message processing error
    ///
    pub async fn run(
        self,
        signal: impl Future<Output = ()> + Send + 'static,
    ) -> Result<(), MessageProcessingError> {
        // When call .await on a &mut _ reference, pin the future. https://docs.rs/tokio/latest/tokio/macro.pin.html#examples
        let signal_future = signal;
        pin!(signal_future);

        loop {
            select! {
                    _ = &mut signal_future => {break;}

                polled_messages = self.consumer().poll_messages(self.poll_command()) => {
                    match polled_messages {
                        Ok(polled_messages) => {
                            for polled_message in polled_messages.messages {
                                self.handle_message(polled_message.payload.as_ref())
                                   .await.expect("Failed to process message");
                            }
                        },
                        Err(e) => {
                            println!("[QDGW/run]: Error polling messages from iggy message bus: {}", e);
                            break;
                        }
                    }
                } // end match polled messages
            } // end select
        } // end loop

        self.shutdown_iggy().await.expect("Failed to shutdown iggy");

        Ok(())
    }
}

impl Server {
    pub(super) async fn shutdown_iggy(&self) -> Result<(), Box<dyn Error>> {
        // Delete consumer stream and topic before shutting down.
        iggy_utils::cleanup(&self.consumer(), &self.iggy_config())
            .await
            .expect("Failed to clean up iggy");

        // Logout user. Call it just once as consumer and producer use the same user.
        iggy_utils::logout_user(&self.consumer())
            .await
            .expect("Failed to logout user");

        // Shutdown consumer
        iggy_utils::shutdown(&self.consumer())
            .await
            .expect("Failed to shutdown iggy consumer");

        // Shutdown producer
        iggy_utils::shutdown(&self.producer())
            .await
            .expect("Failed to shutdown iggy producer");

        Ok(())
    }
}

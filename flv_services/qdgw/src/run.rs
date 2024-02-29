use crate::service::Server;
use common::prelude::{IggyUser, MessageProcessingError};
use iggy::client::MessageClient;
use std::future::Future;
use tokio::{pin, select};

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

        // move into constructor
        let client = iggy_utils::get_iggy_client()
            .await
            .expect("Failed to create client");

        // get user or token from config file
        let user = IggyUser::default();
        // rename to init consumer
        iggy_utils::init_client(&client, &user)
            .await
            .expect("Failed to initialize iggy");

        loop {
            select! {
                    _ = &mut signal_future => {break;}

                polled_messages = client.poll_messages(self.poll_command()) => {
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

        // Shutdown iggy
        iggy_utils::shutdown(&client)
            .await
            .expect("Failed to shutdown iggy");

        Ok(())
    }
}

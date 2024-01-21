use crate::{QDClient};

use async_stream::stream;
use fluvio::Offset;
use futures::stream::StreamExt;

impl QDClient {
    pub async fn on_message<W>(
        &self,
        f: impl Fn(&[u8]) -> W,
    )
        -> impl futures::Stream<Item=W::Output>
        where
            W: futures::Future<Output=()> + Send + 'static,
            W::Output: Send,
    {
        // Create stream for topic consumer.
        let mut stream = self
            .consumer
            .stream(Offset::end())
            .await
            .expect("Failed to create a stream");

        stream! {

            while let Some(Ok(record)) = stream.next().await {
              let value = record.get_value().to_vec();
              let buffer = value.as_slice();

                // Don't really need to spawn a task here since we are not doing anything to heavy.
                // tokio::spawn(f(buffer));

                 f(buffer);
            }
        }
    }
}

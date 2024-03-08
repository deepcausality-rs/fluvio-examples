use iggy::clients::client::IggyClient;
use iggy::messages::poll_messages::PollMessages;
use common::prelude::{IggyConfig, MessageClientConfig};
use crate::QDClient;

impl QDClient {
    pub fn client_id(&self) -> u16 {
        self.client_id
    }
    pub fn producer(&self) -> &IggyClient {
        &self.producer
    }
    pub fn consumer(&self) -> &IggyClient {
        &self.consumer
    }
    pub fn poll_command(&self) -> &PollMessages {
        &self.poll_command
    }
    pub fn client_config(&self) -> &MessageClientConfig {
        &self.client_config
    }

    pub fn iggy_config(&self) -> &IggyConfig {
        &self.iggy_config
    }
}

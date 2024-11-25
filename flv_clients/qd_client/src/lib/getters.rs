use iggy::clients::client::IggyClient;
use iggy::messages::poll_messages::PollMessages;

use common::prelude::IggyConfig;

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
    pub fn consumer_config(&self) -> &IggyConfig {
        &self.consumer_config
    }
    pub fn producer_config(&self) -> &IggyConfig {
        &self.producer_config
    }
}

use iggy::identifier::Identifier;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct IggyConfig {
    stream_id: Identifier,
    stream_name: String,
    topic_id: Identifier,
    topic_name: String,
    tcp_server_addr: String,
    partition_id: Option<u32>,
    messages_per_batch: u32,
    auto_commit: bool,
}

impl IggyConfig {
    pub fn new(
        tcp_server_addr: &str,
        stream_id: Identifier,
        stream_name: String,
        topic_id: Identifier,
        topic_name: String,
        partition_id: Option<u32>,
        messages_per_batch: u32,
        auto_commit: bool,
    ) -> Self {
        Self {
            stream_id,
            stream_name,
            topic_id,
            topic_name,
            tcp_server_addr: tcp_server_addr.to_owned(),
            partition_id,
            messages_per_batch,
            auto_commit,
        }
    }

    pub fn from_client_id(
        tcp_server_addr: &str,
        client_id: u32,
        messages_per_batch: u32,
        auto_commit: bool,
    ) -> Self {
        Self {
            stream_id: Identifier::numeric(client_id).unwrap(),
            stream_name: format!("stream_{}", client_id),
            topic_id: Identifier::numeric(client_id).unwrap(),
            topic_name: format!("topic_{}", client_id),
            tcp_server_addr: tcp_server_addr.to_owned(),
            partition_id: Some(client_id),
            messages_per_batch,
            auto_commit,
        }
    }
}

impl IggyConfig {
    pub fn stream_id(&self) -> Identifier {
        self.stream_id.to_owned()
    }
    pub fn stream_name(&self) -> &str {
        &self.stream_name
    }
    pub fn topic_id(&self) -> Identifier {
        self.topic_id.to_owned()
    }
    pub fn topic_name(&self) -> &str {
        &self.topic_name
    }
    pub fn partition_id(&self) -> Option<u32> {
        self.partition_id
    }
    pub fn messages_per_batch(&self) -> u32 {
        self.messages_per_batch
    }
    pub fn auto_commit(&self) -> bool {
        self.auto_commit
    }

    pub fn tcp_server_addr(&self) -> String {
        self.tcp_server_addr.to_owned()
    }
}

impl Display for IggyConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "IggyConfig: \
             tcp_server_addr: {}, stream_id: {}, stream_name: {}, topic_id: {}, topic_name: {}, \
             partition_id: {}, messages_per_batch: {}, auto_commit: {}",
            self.tcp_server_addr,
            self.stream_id,
            self.stream_name,
            self.topic_id,
            self.topic_name,
            self.partition_id.unwrap_or(0),
            self.messages_per_batch,
            self.auto_commit,
        )
    }
}

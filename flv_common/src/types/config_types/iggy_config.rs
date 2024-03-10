use std::fmt::{Display, Formatter};

use iggy::identifier::Identifier;
use serde::{Deserialize, Serialize};

use crate::prelude::IggyUser;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct IggyConfig {
    user: IggyUser,
    stream_id: Identifier,
    stream_name: String,
    topic_id: Identifier,
    topic_name: String,
    tcp_server_addr: String,
    partition_id: u32,
    messages_per_batch: u32,
    auto_commit: bool,
}

impl IggyConfig {
    pub fn new(
        user: IggyUser,
        tcp_server_addr: &str,
        stream_id: Identifier,
        stream_name: String,
        topic_id: Identifier,
        topic_name: String,
        partition_id: u32,
        messages_per_batch: u32,
        auto_commit: bool,
    ) -> Self {
        Self {
            user,
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
        user: IggyUser,
        client_id: u32,
        messages_per_batch: u32,
        auto_commit: bool,
    ) -> Self {
        Self {
            user,
            stream_id: Identifier::numeric(client_id).unwrap(),
            stream_name: format!("stream_{}", client_id),
            topic_id: Identifier::numeric(client_id).unwrap(),
            topic_name: format!("topic_{}", client_id),
            tcp_server_addr: "127.0.0.1:8090".to_owned(),
            partition_id: client_id,
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
    pub fn partition_id(&self) -> u32 {
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
    pub fn user(&self) -> &IggyUser {
        &self.user
    }
}

impl Display for IggyConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "IggyConfig: \
            iggy_user: {} tcp_server_addr: {}, stream_id: {}, stream_name: {}, topic_id: {}, topic_name: {}, \
             partition_id: {}, messages_per_batch: {}, auto_commit: {}",
            self.user.username(),
            self.tcp_server_addr,
            self.stream_id,
            self.stream_name,
            self.topic_id,
            self.topic_name,
            self.partition_id,
            self.messages_per_batch,
            self.auto_commit,
        )
    }
}

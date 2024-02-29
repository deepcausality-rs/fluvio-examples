use iggy::identifier::Identifier;
use std::fmt::{Display, Formatter};

pub struct IggyConfig {
    stream_id: Identifier,
    topic_id: Identifier,
    partition_id: Option<u32>,
    messages_per_batch: u32,
    auto_commit: bool,
}

impl IggyConfig {
    pub fn new(
        stream_id: Identifier,
        topic_id: Identifier,
        partition_id: Option<u32>,
        messages_per_batch: u32,
        auto_commit: bool,
    ) -> Self {
        Self {
            stream_id,
            topic_id,
            partition_id,
            messages_per_batch,
            auto_commit,
        }
    }

    pub fn from_client_id(client_id: u32, messages_per_batch: u32, auto_commit: bool) -> Self {
        Self {
            stream_id: Identifier::numeric(client_id).unwrap(),
            topic_id: Identifier::numeric(client_id).unwrap(),
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
    pub fn topic_id(&self) -> Identifier {
        self.topic_id.to_owned()
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
}

impl Display for IggyConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,
               "stream_id: {}, topic_id: {}, partition_id: {:?}, messages_per_batch: {}, autocommit: {}",
               self.stream_id, self.topic_id, self.partition_id, self.messages_per_batch, self.auto_commit
        )
    }
}

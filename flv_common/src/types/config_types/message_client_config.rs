use crate::prelude::ServiceID;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MessageClientConfig {
    id: u16,
    name: String,
}

const NAME: &str = "client";

impl MessageClientConfig {
    pub fn new(id: u16) -> Self {
        // Prevents ID clash with configurations generated from ServiceID ENUM
        assert!(id > 20, "id must be greater than 20");
        let name = format!("{}-{}", NAME, id);

        Self { id, name }
    }

    pub fn from_svc_id(svc_id: ServiceID) -> Self {
        let id = svc_id.id().into();
        let svc_name = svc_id.name();

        // Prevents ID clash with manually created configurations
        assert!(id < 20, "id must be less than 20");
        let name = svc_name.to_lowercase().to_string();

        Self { id, name }
    }
}

impl Default for MessageClientConfig {
    fn default() -> Self {
        Self {
            id: 100,
            name: String::from("default-client-100"),
        }
    }
}

impl MessageClientConfig {
    pub fn control_channel(&self) -> String {
        format!("{}-{}", self.name, "control")
    }

    pub fn data_channel(&self) -> String {
        format!("{}-{}", self.name, "data")
    }

    pub fn execution_channel(&self) -> String {
        format!("{}-{}", self.name, "execution")
    }

    pub fn heartbeat_channel(&self) -> String {
        format!("{}-{}", self.name, "heartbeat")
    }
}

impl MessageClientConfig {
    pub fn id(&self) -> u16 {
        self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl Display for MessageClientConfig {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "MessageClientConfig {{ id: {}, name: {} }}",
            self.id, self.name
        )
    }
}

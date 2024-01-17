use crate::prelude::ServiceID;
use std::fmt;

/// The MessageClientConfig struct represents the configuration for a message client.
///
/// It contains the following fields:
///
/// - id: A unique u16 identifier for the client.
///
/// - name: A String name for the client.
///
/// The NAME constant defines a base name that will be used when generating names.
///
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MessageClientConfig {
    id: u16,
    name: String,
}

const NAME: &str = "client";

impl MessageClientConfig {
    /// Creates a new MessageClientConfig with the given id.
    ///
    /// # Parameters
    ///
    /// - `id`: The unique id for the client config. Must be greater than 20 to avoid
    ///   clashing with ids generated from ServiceID.
    ///
    /// # Returns
    ///
    /// A new MessageClientConfig with the given id and a generated name based on the id.
    ///
    pub fn new(id: u16) -> Self {
        // Prevents ID clash with configurations generated from ServiceID ENUM
        assert!(id > 20, "id must be greater than 20");
        let name = format!("{}-{}", NAME, id);

        Self { id, name }
    }

    /// Creates a new MessageClientConfig from a ServiceID.
    ///
    /// Uses the ServiceID's id and name to populate the config fields.
    ///
    /// # Parameters
    ///
    /// - `svc_id`: The ServiceID to create the config from.
    ///
    /// # Returns
    ///
    /// A new MessageClientConfig with an id and name derived from the ServiceID.
    /// The id is asserted to be less than 20 to avoid clashing with manually created configs.
    ///
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
    /// Creates a default MessageClientConfig.
    ///
    /// The default config has an id of 100 and a name of
    /// "default-client-100".
    ///
    /// # Returns
    ///
    /// The default MessageClientConfig.
    fn default() -> Self {
        Self {
            id: 100,
            name: String::from("default-client-100"),
        }
    }
}

impl MessageClientConfig {
    /// Generates a channel name for the control channel based on the client name.
    ///
    /// # Returns
    ///
    /// A String in the format: "{client_name}-control".
    ///
    pub fn control_channel(&self) -> String {
        format!("{}-{}", self.name, "control")
    }

    /// Generates a channel name for the data channel based on the client name.
    ///
    /// # Returns
    ///
    /// A String in the format: "{client_name}-data".
    ///
    pub fn data_channel(&self) -> String {
        format!("{}-{}", self.name, "data")
    }

    /// Generates a channel name for the error channel based on the client name.
    ///
    /// # Returns
    ///
    /// A String in the format: "{client_name}-error".
    pub fn error_channel(&self) -> String {
        format!("{}-{}", self.name, "error")
    }

    /// Generates a channel name for the execution channel based on the client name.
    ///
    /// # Returns
    ///
    /// A String in the format: "{client_name}-execution".
    pub fn execution_channel(&self) -> String {
        format!("{}-{}", self.name, "execution")
    }

    /// Generates a channel name for the heartbeat channel based on the client name.
    ///
    /// # Returns
    ///
    /// A String in the format: "{client_name}-heartbeat".
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

impl fmt::Display for MessageClientConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "MessageClientConfig {{ id: {}, name: {} }}",
            self.id, self.name
        )
    }
}

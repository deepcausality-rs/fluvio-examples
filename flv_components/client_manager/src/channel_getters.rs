use crate::ClientManager;
use common::prelude::MessageClientConfigError;

impl ClientManager {
    ///  Get the control channel for the client with the given ID.
    ///
    ///  # Arguments
    ///
    ///  * `id` - The client ID to look up
    ///
    ///  # Returns
    ///
    ///  Result with the control channel string if the client exists.
    ///  Err with a custom error if the client does not exist.
    ///
    /// # Example
    ///
    /// ```rust
    /// use client_manager::ClientManager;
    /// use common::prelude::MessageClientConfig;
    ///
    ///   let mut manager = ClientManager::new();
    ///
    ///   let id = 100;
    ///     let config = MessageClientConfig::new(id);
    ///     manager.add_client(id, config.clone()).expect("Failed to add client");
    ///
    ///     let result = manager.get_client_control_channel(id);
    ///
    ///     assert!(result.is_ok());
    /// ```
    pub fn get_client_control_channel(&self, id: u16) -> Result<String, MessageClientConfigError> {
        match self.get_client_config(id) {
            Ok(client_config) => Ok(client_config.control_channel().clone()),
            Err(_) => Err(MessageClientConfigError(format!(
                "Client id {} does not exist",
                id
            ))),
        }
    }

    ///  Get the data channel for the client with the given ID.
    ///
    ///  # Arguments
    ///
    ///  * `id` - The client ID to look up
    ///
    ///  # Returns
    ///
    ///  Result with the data channel string if the client exists.
    ///  Err with a custom error if the client does not exist.
    ///
    /// # Example
    ///
    /// ```rust
    /// use client_manager::ClientManager;
    /// use common::prelude::MessageClientConfig;
    ///
    ///   let mut manager = ClientManager::new();
    ///
    ///   let id = 100;
    ///     let config = MessageClientConfig::new(id);
    ///     manager.add_client(id, config.clone()).expect("Failed to add client");
    ///
    ///     let result = manager.get_client_data_channel(id);
    ///
    ///     assert!(result.is_ok());
    /// ```
    pub fn get_client_data_channel(&self, id: u16) -> Result<String, MessageClientConfigError> {
        match self.get_client_config(id) {
            Ok(client_config) => Ok(client_config.data_channel().clone()),
            Err(_) => Err(MessageClientConfigError(format!(
                "Client id {} does not exist",
                id
            ))),
        }
    }

    ///  Get the execution channel for the client with the given ID.
    ///
    ///  # Arguments
    ///
    ///  * `id` - The client ID to look up
    ///
    ///  # Returns
    ///
    ///  Result with the execution channel string if the client exists.
    ///  Err with a custom error if the client does not exist.
    ///
    /// # Example
    ///
    /// ```rust
    /// use client_manager::ClientManager;
    /// use common::prelude::MessageClientConfig;
    ///
    ///   let mut manager = ClientManager::new();
    ///
    ///   let id = 100;
    ///     let config = MessageClientConfig::new(id);
    ///     manager.add_client(id, config.clone()).expect("Failed to add client");
    ///
    ///     let result = manager.get_client_execution_channel(id);
    ///
    ///     assert!(result.is_ok());
    /// ```
    pub fn get_client_execution_channel(
        &self,
        id: u16,
    ) -> Result<String, MessageClientConfigError> {
        match self.get_client_config(id) {
            Ok(client_config) => Ok(client_config.execution_channel().clone()),
            Err(_) => Err(MessageClientConfigError(format!(
                "Client id {} does not exist",
                id
            ))),
        }
    }

    /// Get the heartbeat channel for the client with the given ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The client ID to look up
    ///
    /// # Returns
    ///
    /// Result with the heartbeat channel string if the client exists.
    /// Err with a custom error if the client does not exist.
    ///
    /// # Example
    ///
    /// ```rust
    /// use client_manager::ClientManager;
    /// use common::prelude::MessageClientConfig;
    ///
    ///   let mut manager = ClientManager::new();
    ///
    ///   let id = 100;
    ///     let config = MessageClientConfig::new(id);
    ///     manager.add_client(id, config.clone()).expect("Failed to add client");
    ///
    ///     let result = manager.get_client_heartbeat_channel(id);
    ///
    ///     assert!(result.is_ok());
    /// ```
    pub fn get_client_heartbeat_channel(
        &self,
        id: u16,
    ) -> Result<String, MessageClientConfigError> {
        match self.get_client_config(id) {
            Ok(client_config) => Ok(client_config.heartbeat_channel().clone()),
            Err(_) => Err(MessageClientConfigError(format!(
                "Client id {} does not exist",
                id
            ))),
        }
    }
}

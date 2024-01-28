use crate::ClientManager;
use common::prelude::{MessageClientConfig, MessageClientConfigError};

impl ClientManager {
    /// Adds a new client to the manager.
    ///
    /// Takes a `u16` id and `MessageClientConfig` config.
    /// Returns a `Result`.
    ///
    /// Checks if the id already exists in the `clients` HashMap.
    /// If so, returns an Err with a "Client id already exists" message.
    ///
    /// Otherwise, inserts the id and config into the HashMap
    /// and returns Ok.
    ///
    /// # Arguments
    ///
    /// * `id` - The id for the new client
    /// * `config` - The client configuration
    ///
    /// # Returns
    ///
    /// * `Ok()` - On success
    /// * `Err(MessageClientConfigError)` - If id already exists
    ///
    /// # Example
    ///
    /// ```rust
    ///  use client_manager::ClientManager;
    ///  use common::prelude::MessageClientConfig;
    ///
    ///   let mut manager = ClientManager::new();
    ///
    ///     let id = 100;
    ///     let config = MessageClientConfig::new(id);
    ///     manager
    ///         .add_client(id, config)
    ///         .expect("Failed to add client");
    ///
    ///     assert!(manager.check_client(id));
    /// ```
    pub fn add_client(
        &mut self,
        id: u16,
        config: MessageClientConfig,
    ) -> Result<(), MessageClientConfigError> {
        if self.clients.contains_key(&id) {
            return Err(MessageClientConfigError("[ClientManager]: Client id already exists".into()));
        }

        self.clients.insert(id, config);

        Ok(())
    }

    /// Gets a client configuration by id.
    ///
    /// Takes a `u16` representing the client id.
    /// Returns a `Result` with the `MessageClientConfig` if found.
    ///
    /// Checks if the provided id exists in the `clients` HashMap.
    /// If not, returns an Err with a "Client id does not exist" message.
    ///
    /// If the id exists, tries to retrieve the config from the HashMap.
    /// If missing for some reason, returns an Err with a "Client not found" message.
    ///
    /// # Arguments
    ///
    /// * `id` - The id of the client to retrieve
    ///
    /// # Returns
    ///
    /// * `Ok(MessageClientConfig)` - The client configuration if found.
    /// * `Err(MessageClientConfigError)` - If client id is invalid or missing.
    ///
    /// # Example
    ///
    /// ```rust
    ///  use client_manager::ClientManager;
    ///  use common::prelude::MessageClientConfig;
    ///
    ///   let mut manager = ClientManager::new();
    ///
    ///     let id = 100;
    ///     let config = MessageClientConfig::new(id);
    ///     manager
    ///         .add_client(id, config)
    ///         .expect("Failed to add client");
    ///
    ///     let res = manager
    ///         .get_client_config(id);
    ///
    ///     assert!(res.is_ok());
    /// ```
    pub fn get_client_config(
        &self,
        id: u16,
    ) -> Result<&MessageClientConfig, MessageClientConfigError> {
        if !self.clients.contains_key(&id) {
            return Err(MessageClientConfigError("[ClientManager]: Client id does not exist".into()));
        }

        self.clients
            .get(&id)
            .ok_or(MessageClientConfigError("[ClientManager]: Client not found".into()))
    }

    /// Checks if a client id exists.
    ///
    /// Takes a `u16` representing the client id.
    /// Returns a `bool` indicating if the id exists.
    ///
    /// Checks if the id exists in the `clients` HashMap.
    ///
    /// # Arguments
    ///
    /// * `id` - The id to check
    ///
    /// # Returns
    ///
    /// * `true` - If the id exists
    /// * `false` - If the id does not exist
    ///
    /// # Example
    ///
    /// ```rust
    ///  use client_manager::ClientManager;
    ///  use common::prelude::MessageClientConfig;
    ///
    ///   let mut manager = ClientManager::new();
    ///
    ///     let id = 100;
    ///     let config = MessageClientConfig::new(id);
    ///
    ///     let id = 23;
    ///     let config = MessageClientConfig::new(id);
    ///     manager.add_client(id, config).unwrap();
    ///
    ///     assert!(manager.check_client(id));
    ///     assert!(!manager.check_client(89));
    /// ```
    pub fn check_client(&self, id: u16) -> bool {
        self.clients.contains_key(&id)
    }

    /// Removes a client by id.
    ///
    /// Takes a `u16` representing the client id to remove.
    ///
    /// Checks if the id exists in the `clients` HashMap.
    /// If so, removes the id and associated config.
    ///
    /// No action taken if id does not exist.
    ///
    /// # Arguments
    ///
    /// * `id` - The id of the client to remove
    ///
    /// # Example
    ///
    /// ```rust
    ///  use client_manager::ClientManager;
    ///  use common::prelude::MessageClientConfig;
    ///
    ///   let mut manager = ClientManager::new();
    ///
    ///     let id = 100;
    ///     let config = MessageClientConfig::new(id);
    ///
    ///     let id = 23;
    ///     let config = MessageClientConfig::new(id);
    ///     manager.add_client(id, config).unwrap();
    ///
    ///     assert!(manager.check_client(id));
    ///
    ///     manager.remove_client(id);
    ///
    ///     assert!(!manager.check_client(id));
    /// ```
    pub fn remove_client(&mut self, id: u16) {
        if self.clients.contains_key(&id) {
            self.clients.remove(&id);
        }
    }
}

use crate::ClientManager;
use common::prelude::{MessageClientConfig, MessageClientConfigError};

impl ClientManager {
    /// Adds a new client to the manager.
    ///
    /// Takes a `u16` id and a `String` name.
    /// Returns a `Result`.
    ///
    /// If the id already exists, returns an Err with
    /// "Client id already exists" message.
    /// Otherwise inserts the id and name into the hashmap
    /// and returns Ok.
    pub fn add_client(
        &mut self,
        id: u16,
        config: MessageClientConfig,
    ) -> Result<(), MessageClientConfigError> {
        if self.clients.contains_key(&id) {
            return Err(MessageClientConfigError("Client id already exists".into()));
        }

        self.clients.insert(id, config);

        Ok(())
    }

    /// Gets a client by id.
    ///
    /// Takes a `u16` id and returns a `Result` with the client `String`.
    ///
    /// If the id does not exist, returns an Err with
    /// "Client id does not exist" message.
    /// Otherwise returns the client String in an Ok.
    pub fn get_client_config(
        &self,
        id: u16,
    ) -> Result<&MessageClientConfig, MessageClientConfigError> {
        if !self.clients.contains_key(&id) {
            return Err(MessageClientConfigError("Client id does not exist".into()));
        }

        self.clients
            .get(&id)
            .ok_or(MessageClientConfigError("Client not found".into()))
    }

    /// Checks if a client id exists.
    ///
    /// Takes a `u16` id and returns a `bool`.
    /// Returns true if the id exists in the hashmap.
    /// Returns false if the id does not exist.
    pub fn check_client(&self, id: u16) -> bool {
        self.clients.contains_key(&id)
    }

    /// Removes a client by id.
    ///
    /// Takes a `u16` id.
    /// If the id exists, removes it from the hashmap.
    /// If it does not exist, does nothing.
    pub fn remove_client(&mut self, id: u16) {
        if self.clients.contains_key(&id) {
            self.clients.remove(&id);
        }
    }
}

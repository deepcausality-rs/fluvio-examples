use common::prelude::MessageClientConfig;
use std::collections::HashMap;

mod channel_getters;
mod client_crud;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ClientManager {
    clients: HashMap<u16, MessageClientConfig>,
}

impl ClientManager {
    /// Construct a new ClientManager.
    ///
    /// This initializes a ClientManager with no configured clients. Clients can be
    /// added later via the provided methods.
    ///
    /// # Returns
    ///
    /// A new ClientManager instance.
    pub fn new() -> Self {
        Self {
            clients: HashMap::new(),
        }
    }
}

mod query_builder;
mod query_symbols;
mod query_utils;

use common::prelude::DBConfig;
use postgres::{Client, NoTls};

pub struct QueryDBManager {
    client: Client,
}

impl QueryDBManager {
    pub fn new(db_config: DBConfig) -> Self {
        let params = db_config.pg_connection_string();

        let client = Client::connect(&params, NoTls).expect("Failed to connect to DB");

        Self { client }
    }
}

impl QueryDBManager {
    pub fn close(self) {
        self.client.close().expect("Failed to close DB");
    }

    pub fn is_close(&self) -> bool {
        self.client.is_closed()
    }
}

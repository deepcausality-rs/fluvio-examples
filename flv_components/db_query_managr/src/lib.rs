use common::prelude::DBConfig;
use postgres::{Client, NoTls};

pub struct QueryDBManager {
    db_config: DBConfig,
    client: Client,
}

impl QueryDBManager {

    pub fn new(db_config: DBConfig) -> Self {

        // https://questdb.io/docs/develop/query-data/#postgresql-wire-protocol
        let params = "user=admin password=quest host=127.0.0.1 port=8812 dbname=qdb";

        // https://docs.rs/tokio-postgres/latest/tokio_postgres/
        let  client = Client::connect(params, NoTls)
            .expect("Failed to connect to DB");


        Self { db_config, client }
    }
}

impl QueryDBManager {

}
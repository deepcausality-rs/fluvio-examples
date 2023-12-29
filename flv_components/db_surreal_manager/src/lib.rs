mod fiat_iso;

use std::fmt::{Debug, Display, Formatter};

use common::prelude::DBConfig;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

#[derive(Clone, Debug)]
pub struct SurrealDBManager {
    db: Surreal<Client>,
    db_config: DBConfig,
}

impl SurrealDBManager {
    pub async fn new(db_config: &DBConfig) -> Self {
        // Extract DB config parameters
        let db_host = db_config.host();
        let db_port = db_config.port();
        let db_address = format!("{}:{}", db_host, db_port);
        //
        let db_ns = db_config.db_namespace();
        let db_name = db_config.db_name();
        //
        let db_user = db_config.username();
        let db_pass = db_config.password();

        // Connect to the server
        let db = Surreal::new::<Ws>(db_address)
            .await
            .expect("Failed to connect to Surreal DB server");

        // Signin as a namespace, database, or user
        db.signin(Root {
            username: db_user,
            password: db_pass,
        })
        .await
        .expect("Failed to sign in to Surreal DB server");

        // Select a specific namespace / database
        db.use_db(db_name).await.expect("Failed to set db name");
        db.use_ns(db_ns).await.expect("Failed to set namespace");

        Self {
            db,
            db_config: db_config.to_owned(),
        }
    }
}

impl Display for SurrealDBManager {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "SurrealDBManager Connection parameter: {:?}",
            self.db_config
        )
    }
}

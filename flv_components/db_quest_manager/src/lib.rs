use common::prelude::DBConfig;
use questdb::{
    ingress::{Buffer, SenderBuilder},
    Result as QuestDBResult,
};
use questdb::ingress::Sender;

pub struct QuestDBManager {
    db_config: DBConfig,
    sender: Sender,
}

impl QuestDBManager {
    pub fn new(db_config: DBConfig) -> Self {

        let host = db_config.host();
        let port = db_config.port();

        let sender = SenderBuilder::new(host, port)
            .connect()
            .expect("Failed to connect to QuestDB");

        Self { db_config, sender }
    }
}



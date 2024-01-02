use common::prelude::DBConfig;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct QuestDBManager {
    db_config: DBConfig,
}

impl QuestDBManager {
    pub fn new(db_config: DBConfig) -> Self {
        Self { db_config }
    }
}

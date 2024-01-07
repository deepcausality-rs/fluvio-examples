use crate::QueryDBManager;

impl QueryDBManager {
    pub(crate) fn build_get_symbol_id_query(&self, symbol_table: &str) -> String {
        format!("SELECT symbol_id, symbol FROM {}", symbol_table)
    }
}

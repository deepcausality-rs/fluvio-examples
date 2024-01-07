use crate::QueryDBManager;
use std::fmt::Error;

impl QueryDBManager {
    pub fn get_all_symbols_with_ids(
        &mut self,
        symbol_table: &str,
    ) -> Result<Vec<(u16, String)>, Error> {
        let query = self.build_get_symbol_id_query(symbol_table);

        let rows = self
            .query(&query)
            .expect("Failed to query all symbols from kraken_symbols table");

        let mut result: Vec<(u16, String)> = Vec::with_capacity(rows.len());

        for row in rows {
            let symbol_id: i64 = row.get(0);
            let symbol: String = row.get(1);

            result.push((symbol_id as u16, symbol));
        }

        Ok(result)
    }
}

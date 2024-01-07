use crate::QueryDBManager;

impl QueryDBManager {
    pub(crate) fn query(&mut self, query: &str) -> Result<Vec<postgres::Row>, postgres::Error> {
        self.client.query(query, &[])
    }
}

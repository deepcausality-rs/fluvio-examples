use crate::types::data_types::trade_bar::TradeBar;

impl std::fmt::Display for TradeBar {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "symbol_id: {}, timestamp: {}: price={}, volume={}",
            self.symbol_id, self.date_time, self.price, self.volume
        )
    }
}

use std::fmt;
use crate::prelude::FirstTradeBar;

impl fmt::Display for FirstTradeBar {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "FirstTradeBar {{ message_type: {}, symbol_id: {} }}",
      self.message_type, self.symbol_id
    )
  }
}

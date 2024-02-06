/// Module containing data subscription messages.
///
/// This includes messages like:
///
/// - StartDataMessage
/// - StopDataMessage  
/// - StartAllDataMessage
/// - StopAllDataMessage
/// - OHLCVBarMessage
/// - TradeBarMessage
///
/// Grouping data subscription messages together keeps them organized  
/// separately from client and error messages.
///
/// The data messages are exposed in the prelude for convenient importing.   
///
/// # Exports
///
/// - `ohlcv_bar` - OHLCVBarMessage
/// - `ohlcv_bar_first` - OHLCVBarFirstMessage
/// - `ohlcv_bar_last` - OHLCVBarLastMessage
/// - `start_data` - StartDataMessage
/// - `stop_data` - StopDataMessage
/// - `start_all_data` - StartAllDataMessage
/// - `stop_all_data` - StopAllDataMessage
/// - `trade_bar` - TradeBarMessage
/// - `trade_bar_first` - TradeBarFirstMessage
/// - `trade_bar_last` - TradeBarLastMessage
///
pub mod ohlcv_bar;
pub mod ohlcv_bar_first;
pub mod ohlcv_bar_last;
pub mod start_data;
pub mod stop_all_data;
pub mod stop_data;
pub mod trade_bar;
pub mod trade_bar_first;
pub mod trade_bar_last;

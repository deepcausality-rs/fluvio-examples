/// Data type definitions
///
/// This module defines core data types used throughout the application,
/// organized into submodules:
///
/// - `ohlcv_bar`: OHLCV (open-high-low-close-volume) bar data.
/// - `sampled_bars`: Grouped OHLCV bars for a time period.
/// - `time_resolution`: Resolution enum for OHLCV bars.
/// - `trade_bar`: Tick/trade data bar.
///
/// These types represent key financial data like trades, quotes, and
/// OHLCV bars. They are used in messages and by other parts of the system.
///
/// The types provide strongly typed representations of the data. They
/// encapsulate validation logic and conversions.
///
/// By centralizing the data type definitions, they can be reused
/// consistently across the system.
pub mod ohlcv_bar;
pub mod sampled_bars;
pub mod time_resolution;
pub mod trade_bar;

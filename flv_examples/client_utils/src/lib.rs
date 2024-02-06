///Client utilities for Fluvio examples
///
///This crate provides common utilities used across Fluvio examples.
///
///It contains functions for:
///
///- Printing formatted output
///- Handling data streams
///- Handling errors
///- Working with symbols
///- Loading historical data
///
///The utilities provide building blocks for writing Fluvio clients
///and consumers in a consistent way.
///
///# Modules
///
///- print_utils - Formatted printing of headers, messages
///- handle_utils - Handle streaming data and errors
///- symbol_utils - Lookup symbol IDs and names
///- data_utils - Functions for loading historical data
///- handle_error_utils - Print formatted errors from streams
///
pub mod data_utils;
pub mod handle_error_utils;
pub mod handle_utils;
pub mod prelude;
pub mod print_utils;
pub mod symbol_utils;

use config_manager::ConfigManager;
use db_query_manager::QueryDBManager;
use std::error::Error;
use symbol_manager::SymbolManager;

const FN_NAME: &str = "client_utils/get_symbol_id";

/// Gets the symbol id for the provided symbol name.
///
/// This queries the database to lookup the symbol id mapping.
/// The symbol manager cache is first checked before querying the database.
///
/// # Arguments
///
/// * `cfg_manager` - Configuration manager instance
/// * `symbol` - The symbol name to lookup
///
/// # Returns
///
/// Returns the u16 symbol id if found.
///
/// # Errors
///
/// Returns a `Box<dyn Error>` if the symbol id could not be found.
///
/// # Example
///
/// ```
/// use client_utils::symbol_utils::get_symbol_id;
/// use common::prelude::{ServiceID};
/// use config_manager::ConfigManager;
///
/// async fn test(){
///
/// use client_utils::data_utils::load_data;
/// let cfg_manager = ConfigManager::new(ServiceID::Default);
/// let symbol = "ethaed";
///
/// let result = get_symbol_id(&cfg_manager, symbol).await;
///
/// assert!(result.is_ok());
/// }
/// ```
///
pub async fn get_symbol_id(
    cfg_manager: &ConfigManager,
    symbol: &str,
) -> Result<u16, Box<dyn Error>> {
    // println!("{FN_NAME}: Loading configuration for QueryDBManager.");
    let default_exchange = cfg_manager.default_exchange();
    let exchanges = cfg_manager.exchanges_id_names().to_owned();

    // println!("{FN_NAME}: Creating a new SymbolManager.");
    let exchange_symbol_table = match cfg_manager.get_symbol_table(default_exchange) {
        Some(table) => table,
        None => {
            println!("{FN_NAME}: Failed to get symbol table for default exchange.");
            return Err(Box::from(format!(
                "{FN_NAME}: Failed to get symbol table for default exchange."
            )));
        }
    };

    // println!("{FN_NAME}: Creating a new QueryDBManager.");
    let db_config = cfg_manager.db_config();
    let mut db_query_manager = match QueryDBManager::new(db_config.clone()).await {
        Ok(dqm) => dqm,
        Err(err) => {
            println!("{FN_NAME}: Failed to create QueryDBManager.");
            return Err(Box::from(err));
        }
    };

    // println!("{FN_NAME}: Get all symbols for the default exchange.");
    let symbols = match db_query_manager
        .get_all_symbols_with_ids(&exchange_symbol_table)
        .await {
        Ok(sym) => sym,
        Err(err) => {
            println!("{FN_NAME}: Failed to get all symbols for the default exchange.");
            return Err(Box::from(err));
        }
    };

    // println!("{FN_NAME}: Creating a new SymbolManager.");
    let mut symbol_manager = SymbolManager::new(symbols, exchanges)
        .expect("[get_symbol_id]: Failed to create SymbolManager instance.");

    // println!("{FN_NAME}: Get symbol id for symbol {}.", symbol);
    let symbol_id = match symbol_manager.get_symbol_id(symbol) {
        Ok(id) => id,
        Err(err) => {
            println!("{FN_NAME}: Failed to get symbol id for symbol {}.", symbol);
            return Err(Box::from(err));
        }
    };

    Ok(symbol_id)
}

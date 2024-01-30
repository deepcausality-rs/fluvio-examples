use std::error::Error;
use config_manager::ConfigManager;
use db_query_manager::QueryDBManager;
use symbol_manager::SymbolManager;

const FN_NAME: &str = "client_utils/get_symbol_id";

pub async fn get_symbol_id(
    cfg_manager: &ConfigManager,
    symbol: &str,
) -> Result<u16, Box<dyn Error>> {
    // println!("{FN_NAME}: Loading configuration for QueryDBManager.");
    let default_exchange = cfg_manager.default_exchange();
    let exchanges = cfg_manager.exchanges_id_names().to_owned();

    let exchange_symbol_table = match cfg_manager
        .get_symbol_table(default_exchange) {
        Some(table) => table,
        None => {
            println!("{FN_NAME}: Failed to get symbol table for default exchange.");
            return Err(Box::try_from(format!("{FN_NAME}: Failed to get symbol table for default exchange.")).unwrap());
        }
    };

    // println!("{FN_NAME}: Creating a new QueryDBManager.");
    let db_config = cfg_manager.db_config();
    let mut db_query_manager = match QueryDBManager::new(db_config.clone())
        .await{
        Ok(dqm) => dqm,
        Err(err) => {
            println!("{FN_NAME}: Failed to create QueryDBManager.");
            return Err(Box::try_from(err).unwrap());
        }
    };

    // println!("{FN_NAME}: Get all symbols for the default exchange.");
    let symbols = match db_query_manager
        .get_all_symbols_with_ids(&exchange_symbol_table)
        .await{
        Ok(sym) => sym,
        Err(err) => {
            println!("{FN_NAME}: Failed to get all symbols for the default exchange.");
            return Err(Box::try_from(err).unwrap());
        }
    };

    // println!("{FN_NAME}: Creating a new SymbolManager.");
    let mut symbol_manager = SymbolManager::new(symbols, exchanges)
        .expect("[get_symbol_id]: Failed to create SymbolManager instance.");

    // println!("{FN_NAME}: Get symbol id for symbol {}.", symbol);
    let symbol_id = match symbol_manager
        .get_symbol_id(symbol) {
        Ok(id) => id,
        Err(err) => {
            println!("{FN_NAME}: Failed to get symbol id for symbol {}.", symbol);
            return Err(Box::try_from(err).unwrap());
        }
    };

    Ok(symbol_id)
}

use autometrics::autometrics;
use common::prelude::LookupError;
use std::sync::{Arc, RwLock};
use tonic::{Request, Response, Status};

use proto::binding::symdb_service_server::SymdbService;
use proto::binding::*;
use symbol_manager::SymbolManager;

const FN_NAME: &str = "[SymdbClient/service]: ";

#[derive(Clone)]
pub struct SYMDBServer {
    symbol_manager: Arc<RwLock<SymbolManager>>,
}

impl SYMDBServer {
    pub fn new(symbol_manager: Arc<RwLock<SymbolManager>>) -> Self {
        Self { symbol_manager }
    }
}

#[tonic::async_trait]
#[autometrics]
impl SymdbService for SYMDBServer {
    /// Looks up the exchange name for the given exchange ID.
    ///
    /// # Arguments
    ///
    /// * `request` - The LookupExchangeNameRequest containing the exchange ID to look up.
    ///
    /// # Returns
    ///
    /// Returns a LookupExchangeNameResponse containing a boolean indicating if the exchange was found,
    /// and the name if found. Returns an error if there was an issue looking up the name.
    ///
    /// # Errors
    ///
    /// May return an internal error if there was an issue looking up the exchange name.
    ///
    async fn lookup_exchange_name(
        &self,
        request: Request<LookupExchangeNameRequest>,
    ) -> Result<Response<LookupExchangeNameResponse>, Status> {
        // Extract fields from request
        let exchange_id = request.into_inner().exchange_id;
        // Lock symbol manager
        let sym_manager = self
            .symbol_manager
            .read()
            .expect("Failed To lock symbol manager");

        // Lock up exchange name & handle error
        return match sym_manager.get_exchange_name(exchange_id as u16) {
            Ok(exchange_name) => Ok(Response::new(LookupExchangeNameResponse { exchange_name })),
            Err(e) => Err(Status::internal(e.to_string())),
        };
    }

    /// Looks up a symbol for a given ID.
    ///
    /// # Arguments
    ///
    /// * `request` - The LookupSymbolRequest containing the symbol ID to look up.
    ///
    /// # Returns
    ///
    /// Returns a LookupSymbolResponse containing the Symbol if found.
    /// Returns an error if the symbol was not found.
    ///
    /// # Errors
    ///
    /// May return an NotFound error if the symbol does not exist.
    /// May return an internal error if there was an issue looking up the symbol.
    ///
    async fn lookup_symbol(
        &self,
        request: Request<LookupSymbolRequest>,
    ) -> Result<Response<LookupSymbolResponse>, Status> {
        // Extract fields from request
        let exchange_id = request.get_ref().exchange_id;
        let symbol_id = request.get_ref().symbol_id;

        // Lock symbol manager
        let mut sym_manager = self
            .symbol_manager
            .write()
            .expect("Failed To lock symbol manager");

        let exchange_name = match sym_manager.get_exchange_name(exchange_id as u16) {
            Ok(exchange_id) => exchange_id,
            Err(e) => {
                let msg = format!("Exchange not found for ID: {}", exchange_id);
                return Err(get_status(msg.as_str(), e));
            }
        };

        return match sym_manager.get_symbol(symbol_id as u16) {
            Ok(symbol) => Ok(Response::new(LookupSymbolResponse {
                exchange_name,
                symbol,
            })),
            Err(e) => {
                let msg = format!("Symbol not found for ID: {}", symbol_id);
                return Err(get_status(msg.as_str(), e));
            }
        };
    }

    /// Looks up the symbol ID for the given symbol name and exchange ID.
    ///
    /// # Arguments
    ///
    /// * `request` - The LookupSymbolIdRequest containing the symbol name and exchange ID.
    ///
    /// # Returns
    ///
    /// Returns a LookupSymbolIdResponse containing the symbol ID if found.
    /// Returns an error if the symbol was not found.
    ///
    /// # Errors
    ///
    /// May return a NotFound error if the symbol does not exist for the given name and exchange.
    /// May return an internal error if there was an issue looking up the symbol ID.
    ///
    async fn lookup_symbol_id(
        &self,
        request: Request<LookupSymbolIdRequest>,
    ) -> Result<Response<LookupSymbolIdResponse>, Status> {
        let exchange_id = request.get_ref().exchange_id;
        let symbol = request.into_inner().symbol;

        // Lock symbol manager
        let mut sym_manager = self
            .symbol_manager
            .write()
            .expect("Failed To lock symbol manager");

        // Lookup exchange name
        let exchange_name = match sym_manager.get_exchange_name(exchange_id as u16) {
            Ok(exchange_id) => exchange_id,
            Err(e) => {
                let msg = format!("Exchange not found for ID: {}", exchange_id);
                return Err(get_status(msg.as_str(), e));
            }
        };

        // Lookup ID for Sy,bol & handle error
        return match sym_manager.get_symbol_id(&symbol) {
            Ok(symbol_id) => Ok(Response::new(LookupSymbolIdResponse {
                exchange_name,
                symbol_id: symbol_id as i32,
            })),
            Err(e) => {
                let msg = format!(
                    "Symbol not found for name: {} and exchange: {}",
                    symbol, exchange_name
                );
                return Err(get_status(msg.as_str(), e));
            }
        };
    }
}

fn get_status(msg: &str, e: LookupError) -> Status {
    Status::internal(format!("{FN_NAME} {msg} because of error: {e}"))
}

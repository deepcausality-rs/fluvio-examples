// use autometrics::autometrics;
use tonic::{Request, Response, Status};

use proto::binding::symdb_service_server::SymdbService;
use proto::binding::*;

#[derive(Clone)]
pub struct SYMDBServer {
}


impl SYMDBServer{
    pub fn new() -> Self {
        Self {}
    }
}


#[tonic::async_trait]
// #[autometrics]
impl SymdbService for SYMDBServer{
    async fn lookup_exchange_name(&self, request: Request<LookupExchangeNameRequest>) -> Result<Response<LookupExchangeNameResponse>, Status> {
        todo!()
    }

    async fn lookup_symbol(&self, request: Request<LookupSymbolRequest>) -> Result<Response<LookupSymbolResponse>, Status> {
        todo!()
    }

    async fn lookup_symbol_id(&self, request: Request<LookupSymbolIdRequest>) -> Result<Response<LookupSymbolIdResponse>, Status> {
        todo!()
    }
}
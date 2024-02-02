use common::prelude::HostEndpoint;

pub(crate) fn get_symdb_config() -> HostEndpoint {
    HostEndpoint::new("0.0.0.0".to_string(), 7070)
}

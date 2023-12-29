#[cfg(test)]
mod tests {
    use db_specs::prelude::{db_config_cluster, db_config_local};

    #[test]
    fn test_db_config_local() {
        let config = db_config_local();

        assert_eq!(config.port(), 8000);
        assert_eq!(config.host(), "0.0.0.0");
        assert_eq!(config.db_name(), "fluvio_db");
        assert_eq!(config.db_namespace(), "datastream");
        assert_eq!(config.username(), "root");
        assert_eq!(config.password(), "root");
    }

    #[test]
    fn test_db_config_cluster() {
        let config = db_config_cluster();

        assert_eq!(config.port(), 8000);
        assert_eq!(config.host(), "surrealdb.namespace.url.cluster");
        assert_eq!(config.db_name(), "fluvio_db");
        assert_eq!(config.db_namespace(), "datastream");
        assert_eq!(config.username(), "root");
        assert_eq!(config.password(), "root");
    }
}

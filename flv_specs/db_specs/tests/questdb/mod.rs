#[cfg(test)]
mod tests {
    use common::prelude::DBConfig;
    use db_specs::prelude::{get_cluster_db_config, get_local_db_config};

    #[test]
    fn test_get_local_db_config() {
        let expected = DBConfig::new(9009, "0.0.0.0".into());
        let actual = get_local_db_config();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_get_cluster_db_config() {
        let expected = DBConfig::new(9009, "questdb.default.svc.cluster.local".into());
        let actual = get_cluster_db_config();
        assert_eq!(expected, actual);
    }
}

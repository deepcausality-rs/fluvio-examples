#[cfg(test)]
mod tests {
    use common::prelude::DBConfig;
    use db_specs::prelude::{get_cluster_db_config, get_local_db_config};

    #[test]
    fn test_get_local_db_config() {
        // https://stackoverflow.com/questions/20778771/what-is-the-difference-between-0-0-0-0-127-0-0-1-and-localhost
        let expected = DBConfig::new(9009, "0.0.0.0".into(), "exchanges".to_string());
        let actual = get_local_db_config();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_get_cluster_db_config() {
        let expected = DBConfig::new(
            9009,
            "questdb.default.svc.cluster.local".into(),
            "exchanges".to_string(),
        );
        let actual = get_cluster_db_config();
        assert_eq!(expected, actual);
    }
}

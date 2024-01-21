use common::prelude::DBConfig;

const PORT: u16 = 9009;

/// Returns a DBConfig for connecting to a local QuestDB instance.
///
/// # Returns
///
/// DBConfig with:
/// - PORT set to 9009
/// - host set to "0.0.0.0"
///
/// # Remarks
///
/// Useful for connecting to a local QuestDB instance.
///
pub fn get_local_db_config() -> DBConfig {
    DBConfig::new(PORT, "0.0.0.0".into())
}

/// Returns a DBConfig for connecting to a QuestDB cluster instance.
///
/// # Returns
///
/// DBConfig with:
/// - PORT set to 9009
/// - host set to "questdb.default.svc.cluster.local"
///
/// # Remarks
///
/// Useful for connecting to a QuestDB cluster instance
/// using Kubernetes service discovery.
///
pub fn get_cluster_db_config() -> DBConfig {
    DBConfig::new(PORT, "questdb.default.svc.cluster.local".into())
}

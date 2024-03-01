use common::prelude::IggyConfig;
use iggy::identifier::Identifier;

const MESSAGES_PER_BATCH: u32 = 50;
const AUTO_COMMIT: bool = false;

pub fn get_local_iggy_config(client_id: u32) -> IggyConfig {
    IggyConfig::from_client_id(client_id, MESSAGES_PER_BATCH, AUTO_COMMIT)
}

pub fn get_cluster_iggy_config(client_id: u32) -> IggyConfig {
    IggyConfig::new(
        "iggy.default.svc.cluster.local",
        Identifier::numeric(client_id).unwrap(),
        format!("stream_{}", client_id),
        Identifier::numeric(client_id).unwrap(),
        format!("topic_{}", client_id),
        client_id,
        MESSAGES_PER_BATCH,
        AUTO_COMMIT,
    )
}

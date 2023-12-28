use crate::types::WorkflowOP;
use crate::{
    handle_login_logout, handle_login_start_data, handle_login_start_data_stop_data,
    handle_login_start_data_stop_data_logout,
};
use std::error::Error;

pub async fn handle_workflow(op: &WorkflowOP) -> Result<(), Box<dyn Error>> {
    match op {
        WorkflowOP::LoginLogout => {
            handle_login_logout::handle().await.unwrap();
        }
        WorkflowOP::LoginStartData => {
            handle_login_start_data::handle().await.unwrap();
        }
        WorkflowOP::LoginStartStopData => {
            handle_login_start_data_stop_data::handle().await.unwrap();
        }
        WorkflowOP::LoginStartStopDataLogout => {
            handle_login_start_data_stop_data_logout::handle()
                .await
                .unwrap();
        }
    }

    Ok(())
}

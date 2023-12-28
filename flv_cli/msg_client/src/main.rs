use msg_manager::handle;
use msg_manager::types::WorkflowOP;
use std::error::Error;

const OP: WorkflowOP = WorkflowOP::LoginLogout;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send>> {
    handle::handle_workflow(&OP)
        .await
        .expect("Failed to handle workflow");

    Ok(())
}

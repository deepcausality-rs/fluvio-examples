use msg_manager::types::WorkflowOP;

#[test]
fn test_workflow_op_display() {
    let login_logout = WorkflowOP::LoginLogout;
    assert_eq!(format!("{}", login_logout), "LoginLogout");

    let login_start_data = WorkflowOP::LoginStartData;
    assert_eq!(format!("{}", login_start_data), "LoginStartData");

    let login_start_stop_data = WorkflowOP::LoginStartStopData;
    assert_eq!(format!("{}", login_start_stop_data), "LoginStartStopData");

    let login_start_stop_data_logout = WorkflowOP::LoginStartStopDataLogout;
    assert_eq!(
        format!("{}", login_start_stop_data_logout),
        "LoginStartStopDataLogout"
    );
}

use common::prelude::ClientChannel;

#[test]
fn test_display() {
    let data = ClientChannel::DataChannel;
    assert_eq!(format!("{}", data), "DataChannel");

    let control = ClientChannel::ControlChannel;
    assert_eq!(format!("{}", control), "ControlChannel");

    let execution = ClientChannel::ExecutionChannel;
    assert_eq!(format!("{}", execution), "ExecutionChannel");

    let heartbeat = ClientChannel::HeartbeatChannel;
    assert_eq!(format!("{}", heartbeat), "HeartbeatChannel");
}
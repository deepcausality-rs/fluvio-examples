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


#[test]
fn test_from_u8() {
    assert_eq!(ClientChannel::from(0), ClientChannel::DataChannel);
    assert_eq!(ClientChannel::from(1), ClientChannel::ControlChannel);
    assert_eq!(ClientChannel::from(2), ClientChannel::ErrorChannel);
    assert_eq!(ClientChannel::from(3), ClientChannel::ExecutionChannel);
    assert_eq!(ClientChannel::from(4), ClientChannel::HeartbeatChannel);
}

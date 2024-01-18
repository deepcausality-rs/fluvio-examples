use sbe_messages::prelude::MessageType;

#[test]
fn test_from_u16() {
    assert_eq!(MessageType::from(0_u16), MessageType::UnknownMessageType);
    assert_eq!(MessageType::from(101_u16), MessageType::ClientLogin);
    assert_eq!(MessageType::from(102_u16), MessageType::ClientLogout);
    assert_eq!(MessageType::from(201_u16), MessageType::StartData);
    assert_eq!(MessageType::from(202_u16), MessageType::StopData);
    assert_eq!(MessageType::from(203_u16), MessageType::StopAllData);
    assert_eq!(MessageType::from(204_u16), MessageType::OHLCBar);
    assert_eq!(MessageType::from(205_u16), MessageType::FirstOHLCBar);
    assert_eq!(MessageType::from(206_u16), MessageType::LastOHLCBar);
    assert_eq!(MessageType::from(801_u16), MessageType::ClientError);
    assert_eq!(MessageType::from(999_u16), MessageType::UnknownMessageType);
}
#[test]
fn test_display() {
    let message_type = MessageType::ClientLogin;
    assert_eq!(format!("{}", message_type), "ClientLogin");

    let message_type = MessageType::ClientLogout;
    assert_eq!(format!("{}", message_type), "ClientLogout");

    let message_type = MessageType::StartData;
    assert_eq!(format!("{}", message_type), "StartData");

    let message_type = MessageType::StopData;
    assert_eq!(format!("{}", message_type), "StopData");

    let message_type = MessageType::StopAllData;
    assert_eq!(format!("{}", message_type), "StopAllData");

    let message_type = MessageType::UnknownMessageType;
    assert_eq!(format!("{}", message_type), "UnknownMessageType");
}

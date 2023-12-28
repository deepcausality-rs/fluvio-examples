use sbe_messages::prelude::MessageType;

#[test]
fn test_from_u8() {
    let message_type = MessageType::from(0x1);
    assert_eq!(message_type, MessageType::ClientLogin);

    let message_type = MessageType::from(0x2);
    assert_eq!(message_type, MessageType::ClientLogout);

    let message_type = MessageType::from(0x3);
    assert_eq!(message_type, MessageType::StartData);

    let message_type = MessageType::from(0x4);
    assert_eq!(message_type, MessageType::StopData);

    let message_type = MessageType::from(0x5);
    assert_eq!(message_type, MessageType::StopAllData);

    let message_type = MessageType::from(0xff);
    assert_eq!(message_type, MessageType::UnknownMessageType);
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

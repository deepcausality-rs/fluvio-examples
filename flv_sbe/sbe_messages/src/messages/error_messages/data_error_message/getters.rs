use crate::messages::error_messages::data_error_message::DataErrorMessage;
use crate::prelude::{DataErrorType, MessageType};

impl DataErrorMessage {
    pub fn message_type(&self) -> MessageType {
        self.message_type
    }
    pub fn client_id(&self) -> u16 {
        self.client_id
    }
    pub fn data_error_type(&self) -> DataErrorType {
        self.data_error_type
    }
}

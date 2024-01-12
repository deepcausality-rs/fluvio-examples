use crate::messages::error_messages::data_error_message::DataErrorMessage;
use std::fmt::{Display, Formatter};

impl Display for DataErrorMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "DataErrorMessage {{ message_type: {:?}, client_id: {}, data_error_type: {:?} }}",
            self.message_type, self.client_id, self.data_error_type
        )
    }
}

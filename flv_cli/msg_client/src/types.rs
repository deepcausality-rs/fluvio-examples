use std::fmt;

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub enum WorkflowOP {
    #[default]
    LoginLogout,
    LoginStartData,
    LoginStartStopData,
    LoginStartStopDataLogout,
}

impl fmt::Display for WorkflowOP {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            WorkflowOP::LoginLogout => write!(f, "LoginLogout"),
            WorkflowOP::LoginStartData => write!(f, "LoginStartData"),
            WorkflowOP::LoginStartStopData => write!(f, "LoginStartStopData"),
            WorkflowOP::LoginStartStopDataLogout => write!(f, "LoginStartStopDataLogout"),
        }
    }
}

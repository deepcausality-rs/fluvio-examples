use std::fmt;

#[derive(Debug, Eq, Clone, PartialEq)]
pub struct IggyUser {
    username: String,
    password: String,
}

impl IggyUser {
    pub fn new(username: &str, password: &str) -> Self {
        Self {
            username: username.to_string(),
            password: password.to_string(),
        }
    }
    pub fn username(&self) -> &str {
        &self.username
    }
    pub fn password(&self) -> &str {
        &self.password
    }
}

impl Default for IggyUser {
    fn default() -> Self {
        Self::new("iggy", "iggy")
    }
}

impl fmt::Display for IggyUser {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "User {{ username: {}, password: {} }}",
            self.username, self.password
        )
    }
}

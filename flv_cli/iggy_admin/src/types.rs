use std::fmt;

#[derive(Debug, Eq, Clone, PartialEq)]
pub(crate) struct User {
    username: String,
    password: String,
}

impl User {
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

impl Default for User {
    fn default() -> Self {
        Self::new("iggy", "iggy")
    }
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "User {{ username: {}, password: {} }}",
               self.username, self.password)
    }
}

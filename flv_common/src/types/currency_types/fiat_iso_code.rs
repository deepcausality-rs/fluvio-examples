use serde::Deserialize;
use serde::Serialize;
use std::fmt;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FiatIsoCode {
    currency: String,
    alphabetic_code: [char; 3],
    iso_code: u16,
}

impl FiatIsoCode {
    pub fn new(currency: String, alphabetic_code: [char; 3], iso_code: u16) -> Self {
        Self {
            currency,
            alphabetic_code,
            iso_code,
        }
    }
}

impl FiatIsoCode {
    pub fn currency(&self) -> &str {
        &self.currency
    }
    pub fn alphabetic_code(&self) -> String {
        // convert alphabetic code from fixed sized char array to string
        self.alphabetic_code.iter().collect()
    }
    pub fn iso_code(&self) -> u16 {
        self.iso_code
    }
}

impl fmt::Display for FiatIsoCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} ({}, {})",
            self.currency,
            self.alphabetic_code(),
            self.iso_code
        )
    }
}

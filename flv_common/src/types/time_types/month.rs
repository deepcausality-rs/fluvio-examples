use serde::{Deserialize, Serialize};

/// The Month enum represents the 12 months of the year.
///
/// It is marked as serializable and deserializable for JSON, and implements
/// Display to convert to a string representation.
///
/// The underlying value is a u8, allowing it to be represented efficiently.
///
/// # Variants
///
/// - NoMonth: A default value representing no month.
/// - January through December: The 12 months of the year.
///
/// # Implementation
///
/// - `From<u8>` - Convert a u8 to a Month enum
/// - `Default` - Default to NoMonth
/// - `Display` - Format as a string like "January"
///
#[derive(Serialize, Deserialize, Default, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Month {
    #[default]
    NoMonth = 0,
    January = 1,
    February = 2,
    March = 3,
    April = 4,
    May = 5,
    June = 6,
    July = 7,
    August = 8,
    September = 9,
    October = 10,
    November = 11,
    December = 12,
}

impl From<u8> for Month {
    fn from(value: u8) -> Self {
        match value {
            0 => Month::NoMonth,
            1 => Month::January,
            2 => Month::February,
            3 => Month::March,
            4 => Month::April,
            5 => Month::May,
            6 => Month::June,
            7 => Month::July,
            8 => Month::August,
            9 => Month::September,
            10 => Month::October,
            11 => Month::November,
            12 => Month::December,
            _ => Month::NoMonth,
        }
    }
}

impl std::fmt::Display for Month {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Month::January => write!(f, "January"),
            Month::February => write!(f, "February"),
            Month::March => write!(f, "March"),
            Month::April => write!(f, "April"),
            Month::May => write!(f, "May"),
            Month::June => write!(f, "June"),
            Month::July => write!(f, "July"),
            Month::August => write!(f, "August"),
            Month::September => write!(f, "September"),
            Month::October => write!(f, "October"),
            Month::November => write!(f, "November"),
            Month::December => write!(f, "December"),
            Month::NoMonth => write!(f, "No Month"),
        }
    }
}

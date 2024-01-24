use std::fmt::{Display, Formatter};

#[derive(Debug, Default, Copy, Clone, Hash, Eq, PartialEq)]
#[repr(u8)]
pub enum TimeScale {
    #[default]
    NoScale = 0,
    Second = 1,
    Minute = 2,
    Hour = 3,
    Day = 4,
    Week = 5,
    Month = 6,
    Quarter = 7,
    Year = 8,
}

impl From<u8> for TimeScale {
    fn from(value: u8) -> Self {
        match value {
            0 => TimeScale::NoScale,
            1 => TimeScale::Second,
            2 => TimeScale::Minute,
            3 => TimeScale::Hour,
            4 => TimeScale::Day,
            5 => TimeScale::Week,
            6 => TimeScale::Month,
            7 => TimeScale::Quarter,
            8 => TimeScale::Year,
            _ => TimeScale::NoScale,
        }
    }
}

impl Display for TimeScale {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

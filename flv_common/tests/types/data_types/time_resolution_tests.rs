use common::prelude::TimeResolution;
use std::str::FromStr;

#[test]
fn test_from_str() {
    assert_eq!(
        TimeResolution::from_str("NoValue"),
        Ok(TimeResolution::NoValue)
    );
    assert_eq!(
        TimeResolution::from_str("OneMin"),
        Ok(TimeResolution::OneMin)
    );
    assert_eq!(
        TimeResolution::from_str("FiveMin"),
        Ok(TimeResolution::FiveMin)
    );
    assert_eq!(
        TimeResolution::from_str("FifteenMin"),
        Ok(TimeResolution::FifteenMin)
    );
    assert_eq!(
        TimeResolution::from_str("ThirtyMin"),
        Ok(TimeResolution::ThirtyMin)
    );
    assert_eq!(
        TimeResolution::from_str("OneHour"),
        Ok(TimeResolution::OneHour)
    );
    assert_eq!(
        TimeResolution::from_str("OneDay"),
        Ok(TimeResolution::OneDay)
    );
    assert_eq!(
        TimeResolution::from_str("OneWeek"),
        Ok(TimeResolution::OneWeek)
    );
    assert_eq!(TimeResolution::from_str("Invalid"), Err(()));
}

#[test]
fn test_from_u8() {
    assert_eq!(TimeResolution::from(0x0_u8), TimeResolution::NoValue);
    assert_eq!(TimeResolution::from(0x1_u8), TimeResolution::OneMin);
    assert_eq!(TimeResolution::from(0x2_u8), TimeResolution::FiveMin);
    assert_eq!(TimeResolution::from(0x3_u8), TimeResolution::FifteenMin);
    assert_eq!(TimeResolution::from(0x4_u8), TimeResolution::ThirtyMin);
    assert_eq!(TimeResolution::from(0x5_u8), TimeResolution::OneHour);
    assert_eq!(TimeResolution::from(0x6_u8), TimeResolution::OneDay);
    assert_eq!(TimeResolution::from(0x7_u8), TimeResolution::OneWeek);
    assert_eq!(TimeResolution::from(0x8_u8), TimeResolution::NoValue);
}

#[test]
fn test_display() {
    assert_eq!(format!("{}", TimeResolution::NoValue), "NoValue");
    assert_eq!(format!("{}", TimeResolution::OneMin), "OneMin");
    assert_eq!(format!("{}", TimeResolution::FiveMin), "FiveMin");
    assert_eq!(format!("{}", TimeResolution::FifteenMin), "FifteenMin");
    assert_eq!(format!("{}", TimeResolution::ThirtyMin), "ThirtyMin");
    assert_eq!(format!("{}", TimeResolution::OneHour), "OneHour");
    assert_eq!(format!("{}", TimeResolution::OneDay), "OneDay");
    assert_eq!(format!("{}", TimeResolution::OneWeek), "OneWeek");
}

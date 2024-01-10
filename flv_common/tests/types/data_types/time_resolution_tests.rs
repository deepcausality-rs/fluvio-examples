use common::prelude::TimeResolution;

#[test]
fn test_from_u8() {
    assert_eq!(TimeResolution::from(0_u8), TimeResolution::NoValue);
    assert_eq!(TimeResolution::from(1_u8), TimeResolution::OneMin);
    assert_eq!(TimeResolution::from(2_u8), TimeResolution::FiveMin);
    assert_eq!(TimeResolution::from(3_u8), TimeResolution::FifteenMin);
    assert_eq!(TimeResolution::from(4_u8), TimeResolution::ThirtyMin);
    assert_eq!(TimeResolution::from(5_u8), TimeResolution::OneHour);
    assert_eq!(TimeResolution::from(6_u8), TimeResolution::OneDay);
    assert_eq!(TimeResolution::from(7_u8), TimeResolution::OneMonth);
    assert_eq!(TimeResolution::from(8_u8), TimeResolution::OneYear);
    assert_eq!(TimeResolution::from(9_u8), TimeResolution::NoValue);
}

#[test]
fn test_display() {
    assert_eq!(TimeResolution::NoValue.to_string(), "NoValue");
    assert_eq!(TimeResolution::OneMin.to_string(), "1m");
    assert_eq!(TimeResolution::FiveMin.to_string(), "5m");
    assert_eq!(TimeResolution::FifteenMin.to_string(), "15m");
    assert_eq!(TimeResolution::ThirtyMin.to_string(), "30m");
    assert_eq!(TimeResolution::OneHour.to_string(), "1h");
    assert_eq!(TimeResolution::OneDay.to_string(), "1d");
    assert_eq!(TimeResolution::OneMonth.to_string(), "1M");
    assert_eq!(TimeResolution::OneYear.to_string(), "1y");
}

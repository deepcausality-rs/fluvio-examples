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
    assert_eq!(TimeResolution::from(7_u8), TimeResolution::OneWeek);
    assert_eq!(TimeResolution::from(8_u8), TimeResolution::OneMonth);
    assert_eq!(TimeResolution::from(9_u8), TimeResolution::OneYear);
}

#[test]
fn test_display() {
    assert_eq!(TimeResolution::NoValue.to_string(), "NoValue");
    assert_eq!(TimeResolution::OneMin.to_string(), "1 minute");
    assert_eq!(TimeResolution::FiveMin.to_string(), "5 minute");
    assert_eq!(TimeResolution::FifteenMin.to_string(), "15 minute");
    assert_eq!(TimeResolution::ThirtyMin.to_string(), "30 minute");
    assert_eq!(TimeResolution::OneHour.to_string(), "1 hour");
    assert_eq!(TimeResolution::OneDay.to_string(), "1 day");
    assert_eq!(TimeResolution::OneMonth.to_string(), "1 month");
    assert_eq!(TimeResolution::OneYear.to_string(), "1 year");
}

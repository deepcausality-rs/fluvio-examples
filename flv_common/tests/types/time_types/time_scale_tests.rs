use common::prelude::TimeScale;

#[test]
fn test_from_u8() {
    assert_eq!(TimeScale::from(0), TimeScale::NoScale);
    assert_eq!(TimeScale::from(1), TimeScale::Second);
    assert_eq!(TimeScale::from(2), TimeScale::Minute);
    assert_eq!(TimeScale::from(3), TimeScale::Hour);
    assert_eq!(TimeScale::from(4), TimeScale::Day);
    assert_eq!(TimeScale::from(5), TimeScale::Week);
    assert_eq!(TimeScale::from(6), TimeScale::Month);
    assert_eq!(TimeScale::from(7), TimeScale::Quarter);
    assert_eq!(TimeScale::from(8), TimeScale::Year);
    assert_eq!(TimeScale::from(9), TimeScale::NoScale);
}

#[test]
fn test_display() {
    let no_scale = TimeScale::NoScale;
    let second = TimeScale::Second;

    assert_eq!(format!("{}", no_scale), "NoScale");
    assert_eq!(format!("{}", second), "Second");
}
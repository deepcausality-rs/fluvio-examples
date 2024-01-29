use deep_causality::prelude::TimeScale;
use causal_model::prelude::time_utils::get_time_scale_control_map;

#[test]
fn test_get_boolean_control_map() {
    let no_scale = get_time_scale_control_map(&TimeScale::NoScale);
    assert_eq!(
        no_scale,
        vec![true, true, true, true, true, true, true, true]
    );

    let second = get_time_scale_control_map(&TimeScale::Second);
    assert_eq!(second, vec![true, true, true, true, true, true, true, true]);

    let minute = get_time_scale_control_map(&TimeScale::Minute);
    assert_eq!(
        minute,
        vec![true, true, true, true, true, true, true, false]
    );

    let hour = get_time_scale_control_map(&TimeScale::Hour);
    assert_eq!(hour, vec![true, true, true, true, true, true, false, false]);

    let day = get_time_scale_control_map(&TimeScale::Day);
    assert_eq!(day, vec![true, true, true, true, true, false, false, false]);

    let week = get_time_scale_control_map(&TimeScale::Week);
    assert_eq!(
        week,
        vec![true, true, true, true, false, false, false, false]
    );

    let month = get_time_scale_control_map(&TimeScale::Month);
    assert_eq!(
        month,
        vec![true, true, true, false, false, false, false, false]
    );

    let quarter = get_time_scale_control_map(&TimeScale::Quarter);
    assert_eq!(
        quarter,
        vec![true, true, false, false, false, false, false, false]
    );

    let year = get_time_scale_control_map(&TimeScale::Year);
    assert_eq!(
        year,
        vec![true, false, false, false, false, false, false, false]
    );
}

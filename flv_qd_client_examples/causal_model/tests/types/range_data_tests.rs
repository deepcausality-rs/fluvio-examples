use causal_model::prelude::{BarRange, RangeData, Rangeable};
use deep_causality::prelude::Identifiable;
use rust_decimal::Decimal;

#[test]
fn test_new() {
    let id = 1;
    let data_range = BarRange::new(
        Decimal::from(70),
        Decimal::from(100),
        Decimal::from(90),
        false,
        true,
    );

    let range_data = RangeData::new(id, data_range);

    assert_eq!(id, range_data.id());
    assert_eq!(data_range, range_data.data_range());
}

#[test]
fn test_display() {
    let id = 1;
    let data_range = BarRange::new(
        Decimal::from(70),
        Decimal::from(100),
        Decimal::from(90),
        false,
        true,
    );
    let range_data = RangeData::new(id, data_range);

    let expected = "id: 1 range: BarRange { open: 70 high: 100, close: 90, close_above_open: false, close_below_open: true }";
    assert_eq!(expected, format!("{}", range_data));
}

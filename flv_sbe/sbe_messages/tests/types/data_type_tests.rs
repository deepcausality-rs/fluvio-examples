use sbe_messages::prelude::DataType;

#[test]
fn test_from_u8() {
    assert_eq!(DataType::from(0), DataType::UnknownDataType);
    assert_eq!(DataType::from(1), DataType::TradeData);
    assert_eq!(DataType::from(2), DataType::OHLCVData);
    assert_eq!(DataType::from(3), DataType::OrderBookData);
    assert_eq!(DataType::from(4), DataType::QuoteData);
    assert_eq!(DataType::from(5), DataType::UnknownDataType);
}

#[test]
fn test_display() {
    assert_eq!(format!("{}", DataType::UnknownDataType), "UnknownDataType");
    assert_eq!(format!("{}", DataType::TradeData), "TradeData");
    assert_eq!(format!("{}", DataType::OHLCVData), "OHLCVData");
    assert_eq!(format!("{}", DataType::OrderBookData), "OrderBookData");
    assert_eq!(format!("{}", DataType::QuoteData), "QuoteData");
}

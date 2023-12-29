use common::prelude::SymbolID;

#[test]
fn test_symbol_id_from() {
    assert_eq!(SymbolID::from(0xff_u16), SymbolID::NullVal);
    assert_eq!(SymbolID::from(0x1_u16), SymbolID::BTCUSD);
    assert_eq!(SymbolID::from(0x2_u16), SymbolID::ETHUSD);
    assert_eq!(SymbolID::from(0x3_u16), SymbolID::LTCUSD);
    assert_eq!(SymbolID::from(0x4_u16), SymbolID::NullVal);
}

#[test]
fn test_symbol_id_display() {
    assert_eq!(format!("{}", SymbolID::NullVal), "NullVal");
    assert_eq!(format!("{}", SymbolID::BTCUSD), "BTCUSD");
    assert_eq!(format!("{}", SymbolID::ETHUSD), "ETHUSD");
    assert_eq!(format!("{}", SymbolID::LTCUSD), "LTCUSD");
}

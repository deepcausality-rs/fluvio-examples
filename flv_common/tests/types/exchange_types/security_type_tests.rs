use common::prelude::SecurityType;
#[test]
fn test_default_variant() {
    let security_type: SecurityType = Default::default();
    assert_eq!(security_type, SecurityType::Spot);
}

#[test]
fn test_unknown_security_type() {
    let security_type: SecurityType = SecurityType::UnknownSecurityType;
    assert_eq!(security_type, SecurityType::UnknownSecurityType);
}

#[test]
fn test_debug() {
    assert_eq!(
        format!("{:?}", SecurityType::UnknownSecurityType),
        "UnknownSecurityType"
    );
    assert_eq!(format!("{:?}", SecurityType::Spot), "Spot");
    assert_eq!(format!("{:?}", SecurityType::Index), "Index");
    assert_eq!(format!("{:?}", SecurityType::Future), "Future");
    assert_eq!(
        format!("{:?}", SecurityType::PerpetualFuture),
        "PerpetualFuture"
    );
    assert_eq!(format!("{:?}", SecurityType::Option), "Option");
    assert_eq!(format!("{:?}", SecurityType::FutureOption), "FutureOption");
}

#[test]
fn test_display() {
    let security_type = SecurityType::UnknownSecurityType;
    assert_eq!(security_type.to_string(), "UnknownSecurityType");

    let security_type = SecurityType::Spot;
    assert_eq!(security_type.to_string(), "Spot");

    let security_type = SecurityType::Index;
    assert_eq!(security_type.to_string(), "Index");

    let security_type = SecurityType::Future;
    assert_eq!(security_type.to_string(), "Future");

    let security_type = SecurityType::PerpetualFuture;
    assert_eq!(security_type.to_string(), "PerpetualFuture");

    let security_type = SecurityType::Option;
    assert_eq!(security_type.to_string(), "Option");

    let security_type = SecurityType::FutureOption;
    assert_eq!(security_type.to_string(), "FutureOption");
}

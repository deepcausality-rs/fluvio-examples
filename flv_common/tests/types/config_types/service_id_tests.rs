use common::prelude::ServiceID;

#[test]
fn test_default() {
    let service_name = ServiceID::default();
    assert_eq!(service_name, ServiceID::Default);
}

#[test]
fn test_from_u8() {
    assert_eq!(ServiceID::from(0x0_u8), ServiceID::Default);
    assert_eq!(ServiceID::from(99_u8), ServiceID::Database);
    assert_eq!(ServiceID::from(0x1_u8), ServiceID::QDGW);
    assert_eq!(ServiceID::from(0x2_u8), ServiceID::SYMDB);
    assert_eq!(ServiceID::from(0x42_u8), ServiceID::Default);
}

#[test]
fn test_debug() {
    let e = ServiceID::Default;
    assert_eq!(format!("{:?}", e), "Default");

    let e = ServiceID::Database;
    assert_eq!(format!("{:?}", e), "Database");

    let e = ServiceID::QDGW;
    assert_eq!(format!("{:?}", e), "QDGW");

    let e = ServiceID::SYMDB;
    assert_eq!(format!("{:?}", e), "SYMDB");
}

#[test]
fn test_from_string() {
    assert_eq!(ServiceID::from_string("Default"), Some(ServiceID::Default));
    assert_eq!(
        ServiceID::from_string("Database"),
        Some(ServiceID::Database)
    );
    assert_eq!(ServiceID::from_string("QDGW"), Some(ServiceID::QDGW));
    assert_eq!(ServiceID::from_string("SYMDB"), Some(ServiceID::SYMDB));
    assert_eq!(ServiceID::from_string("Unknown"), None);
}

#[test]
fn test_display() {
    assert_eq!(format!("{}", ServiceID::Default), "Default");
    assert_eq!(format!("{}", ServiceID::Database), "Database");
    assert_eq!(format!("{}", ServiceID::QDGW), "QDGW");
    assert_eq!(format!("{}", ServiceID::SYMDB), "SYMDB");
}

use common::prelude::EnvironmentType;

#[test]
fn test_from_u8() {
    let local = EnvironmentType::from(0);
    assert_eq!(local, EnvironmentType::Local);

    let cluster = EnvironmentType::from(1);
    assert_eq!(cluster, EnvironmentType::Cluster);
}

#[test]
fn test_from_string() {
    let result = EnvironmentType::from_string("local");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), EnvironmentType::Local);

    let result = EnvironmentType::from_string("invalid");
    assert!(result.is_err());
}

#[test]
fn test_debug() {
    let local = EnvironmentType::Local;
    assert_eq!(format!("{:?}", local), "Local");

    let cluster = EnvironmentType::Cluster;
    assert_eq!(format!("{:?}", cluster), "Cluster");
}

#[test]
fn test_display() {
    let local = EnvironmentType::Local;
    assert_eq!(format!("{}", local), "Local");

    let cluster = EnvironmentType::Cluster;
    assert_eq!(format!("{}", cluster), "Cluster");
}

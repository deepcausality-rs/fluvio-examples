use client_utils::prelude::atomic_counter::RelaxedAtomicCounter;

#[test]
fn test_increment() {
    let counter = RelaxedAtomicCounter::new();

    let v1 = counter.increment_and_get();
    assert_eq!(v1, 1);

    let v2 = counter.increment_and_get();
    assert_eq!(v2, 2);

    let v3 = counter.increment_and_get();
    assert_eq!(v3, 3);
}

#[test]
fn test_display() {
    let counter = RelaxedAtomicCounter::new();

    counter.increment_and_get();

    assert_eq!(format!("{}", counter), "1");

    counter.increment_and_get();

    assert_eq!(format!("{}", counter), "2");
}

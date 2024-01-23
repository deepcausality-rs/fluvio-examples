use deep_causality::prelude::{Identifiable};
use lib_inference::prelude::causaloid_range_month::get_current_month_causaloid;
use lib_inference::types::alias::CustomContext;

#[test]
fn test_get_current_month_causaloid() {
    // Arrange
    let context = CustomContext::with_capacity(1, "Test ctx", 10);
    let id = 23;

    // Act
    let causaloid = get_current_month_causaloid(&context, id);

    // Assert
    assert!(!causaloid.active());
    assert_eq!(causaloid.id(), id);
    assert_eq!(causaloid.description(), "Checks if the current price exceeds the range high level of the current month");
}
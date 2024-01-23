use common::prelude::Month;

#[test]
fn test_month_variants() {
    let jan = Month::January;
    let feb = Month::February;
    let mar = Month::March;
    let apr = Month::April;
    let may = Month::May;
    let jun = Month::June;
    let jul = Month::July;
    let aug = Month::August;
    let sep = Month::September;
    let oct = Month::October;
    let nov = Month::November;
    let dec = Month::December;

    assert_eq!(jan as u8, 1);
    assert_eq!(feb as u8, 2);
    assert_eq!(mar as u8, 3);
    assert_eq!(apr as u8, 4);
    assert_eq!(may as u8, 5);
    assert_eq!(jun as u8, 6);
    assert_eq!(jul as u8, 7);
    assert_eq!(aug as u8, 8);
    assert_eq!(sep as u8, 9);
    assert_eq!(oct as u8, 10);
    assert_eq!(nov as u8, 11);
    assert_eq!(dec as u8, 12);
}


#[test]
fn test_from_u8() {
    assert_eq!(Month::from(0), Month::NoMonth);
    assert_eq!(Month::from(1), Month::January);
    assert_eq!(Month::from(2), Month::February);
    assert_eq!(Month::from(3), Month::March);
    assert_eq!(Month::from(4), Month::April);
    assert_eq!(Month::from(5), Month::May);
    assert_eq!(Month::from(6), Month::June);
    assert_eq!(Month::from(7), Month::July);
    assert_eq!(Month::from(8), Month::August);
    assert_eq!(Month::from(9), Month::September);
    assert_eq!(Month::from(10), Month::October);
    assert_eq!(Month::from(11), Month::November);
    assert_eq!(Month::from(12), Month::December);
    assert_eq!(Month::from(13), Month::NoMonth);
}

#[test]
fn test_default() {
    let default_month = Month::default();

    assert_eq!(default_month, Month::NoMonth);
}

#[test]
fn test_display() {
    let jan = Month::January;
    let feb = Month::February;
    let mar = Month::March;
    let apr = Month::April;
    let may = Month::May;
    let jun = Month::June;
    let jul = Month::July;
    let aug = Month::August;
    let sep = Month::September;
    let oct = Month::October;
    let nov = Month::November;
    let dec = Month::December;
    let no_month = Month::NoMonth;

    assert_eq!(format!("{}", jan), "January");
    assert_eq!(format!("{}", feb), "February");
    assert_eq!(format!("{}", mar), "March");
    assert_eq!(format!("{}", apr), "April");
    assert_eq!(format!("{}", may), "May");
    assert_eq!(format!("{}", jun), "June");
    assert_eq!(format!("{}", jul), "July");
    assert_eq!(format!("{}", aug), "August");
    assert_eq!(format!("{}", sep), "September");
    assert_eq!(format!("{}", oct), "October");
    assert_eq!(format!("{}", nov), "November");
    assert_eq!(format!("{}", dec), "December");
    assert_eq!(format!("{}", no_month), "No Month");
}
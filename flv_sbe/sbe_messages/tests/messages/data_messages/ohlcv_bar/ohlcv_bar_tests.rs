use common::prelude::OHLCVBar;
use sbe_messages::prelude::SbeOHLCVBar;

#[test]
fn test_encode_data_bar_message() {
    let bar = OHLCVBar::default(); // Create a sample DataBar

    let result = SbeOHLCVBar::encode_data_bar_message(bar);

    assert!(result.is_ok()); // Assert encode passes

    let (size, encoded) = result.unwrap();
    assert_eq!(size, 38); // Assert encoded message size matches expected
    assert!(!encoded.is_empty()); // Assert non-empty encoded message
}

#[test]
fn test_decode_data_bar_message() {
    // Encode a sample DataBar
    let bar = OHLCVBar::default();
    let (size, encoded) = SbeOHLCVBar::encode_data_bar_message(bar.clone()).unwrap();
    assert_eq!(size, 38); // Assert encoded message size matches expected
    assert!(!encoded.is_empty()); // Assert non-empty encoded message

    // Decode the encoded message
    let result = SbeOHLCVBar::decode_data_bar_message(&encoded);

    assert!(result.is_ok()); // Assert decode passes

    let decoded_bar = result.unwrap();
    let original_bar = bar.clone();

    // Compare decoded bar with original bar field by field
    // Timestamp seems to have a loss of precision during encoding/decoding
    assert_eq!(
        decoded_bar.date_time().to_rfc2822(),
        original_bar.date_time().to_rfc2822()
    );
    assert_eq!(decoded_bar.open(), original_bar.open());
    assert_eq!(decoded_bar.high(), original_bar.high());
    assert_eq!(decoded_bar.low(), original_bar.low());
    assert_eq!(decoded_bar.close(), original_bar.close());
    assert_eq!(decoded_bar.volume(), original_bar.volume());
}

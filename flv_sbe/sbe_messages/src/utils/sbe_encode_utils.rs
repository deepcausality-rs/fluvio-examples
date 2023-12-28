use crate::errors::SbeEncodeError;

/// Encodes a string into a fixed-length array of bytes.
///
/// # Errors
///
/// Returns an error if the input string is empty or longer than the specified target byte length.
///
/// # Panics
/// This function does not panic.
#[inline]
pub fn encode_string<const N: usize>(str: &str) -> Result<[u8; N], SbeEncodeError> {
    // Check if the input string is empty.
    if str.is_empty() {
        return Err(SbeEncodeError(
            "String is empty. Cannot encode null length string.".into(),
        ));
    }

    // Check if the input string is longer than the specified target byte length.
    if str.len() > N {
        return Err(SbeEncodeError(format!(
            "String is longer than {} target byte length. Cannot encode string.",
            N
        )));
    }

    // Convert the input string into a byte slice.
    let bytes = str.as_bytes();

    // Try to convert the byte slice into a fixed-length array of bytes.
    match <[u8; N]>::try_from(bytes) {
        // Return the fixed-length array of bytes if successful.
        Ok(bytes) => Ok(bytes),
        // Return an error if the conversion fails.
        Err(e) => Err(SbeEncodeError(e.to_string())),
    }
}

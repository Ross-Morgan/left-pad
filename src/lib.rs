/// Left pads a string with char to a specified length
///
/// If len <= s.len(), then string is not modified
/// # Examples
/// ```
/// # use left_pad::left_pad;
/// let before = String::from("Hello");
/// let after = left_pad(before.as_str(), 7, '=');
///
/// assert_eq!(after, "==Hello");
/// ```
pub fn left_pad(s: &str, len: usize, pad_char: char) -> String {
    (0..(len.saturating_sub(s.len())))
        .map(|_| pad_char)
        .chain(s.chars())
        .collect()
}

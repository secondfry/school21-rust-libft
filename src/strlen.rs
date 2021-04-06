/// # strlen
/// Returns str.len().
///
/// This function has no reason to exist.
///
/// ## Example
/// ```
/// assert_eq!(ft::strlen("abc"), 3);
/// assert_eq!(ft::strlen("\0bc"), 3);
/// ```
pub fn strlen(s: &str) -> usize {
  return s.len();
}

/// # strlen_naive
/// Returns amount of sumbols to first null-symbol in the string.
///
/// ## Example
/// ```
/// assert_eq!(ft::strlen_naive("abc"), 3);
/// assert_eq!(ft::strlen_naive("a\0c"), 1);
/// ```
pub fn strlen_naive(s: &str) -> usize {
  let c: Vec<char> = s.chars().collect();
  let mut i: usize = 0;

  while i < c.len() && c[i] != '\0' {
    i += 1;
  }

  return i;
}

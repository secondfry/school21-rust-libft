#[test]
fn strlen_works() {
  assert_eq!(ft::strlen(""), 0);
  assert_eq!(ft::strlen("\0"), 1);
  assert_eq!(ft::strlen("a"), 1);
  assert_eq!(ft::strlen("ab"), 2);
  assert_eq!(ft::strlen("abc"), 3);
  assert_eq!(ft::strlen("\0bc"), 3);
}

#[test]
fn strlen_naive_works() {
  assert_eq!(ft::strlen_naive(""), 0);
  assert_eq!(ft::strlen_naive("\0"), 0);
  assert_eq!(ft::strlen_naive("a"), 1);
  assert_eq!(ft::strlen_naive("ab"), 2);
  assert_eq!(ft::strlen_naive("abc"), 3);
  assert_eq!(ft::strlen_naive("\0bc"), 0);
  assert_eq!(ft::strlen_naive("a\0c"), 1);
  assert_eq!(ft::strlen_naive("ab\0"), 2);
}

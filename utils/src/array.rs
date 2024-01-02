
/// To get the last element of an array with index -1, or the first with index len.
const fn wrapping_index(i: i32, len: usize) -> usize {
    // https://stackoverflow.com/a/45397704
    let c = len as i32;
    ((i % c + c) % c) as usize
}

#[test]
fn test_wrapping_index() {
    assert_eq!(wrapping_index(-1, 6), 5);
    assert_eq!(wrapping_index(0, 6), 0);
    assert_eq!(wrapping_index(3, 6), 3);
    assert_eq!(wrapping_index(6, 6), 0);
}
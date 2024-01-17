/// To get the last element of an array with index -1, or the first with index len.
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_possible_wrap)]
#[allow(clippy::cast_sign_loss)]
const fn wrapping(i: i32, len: usize) -> usize {
    // https://stackoverflow.com/a/45397704
    let c = len as i32;
    ((i % c + c) % c) as usize
}

const fn wrapping_index(i: usize, len: usize) -> usize {
    (i % len + len) % len
}

#[test]
fn test_wrapping_index() {
    assert_eq!(wrapping(-1, 6), 5);
    assert_eq!(wrapping(0, 6), 0);
    assert_eq!(wrapping(3, 6), 3);
    assert_eq!(wrapping(6, 6), 0);
}

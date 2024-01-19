/// To get the last element of an array with index -1, or the first with index len.
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_possible_wrap)]
const fn wrapping(i: i32, len: usize) -> usize {
    i.rem_euclid(len as i32) as usize
}

const fn wrapping_index(i: usize, len: usize) -> usize {
    i.rem_euclid(len)
}

#[test]
fn test_wrapping_index() {
    assert_eq!(wrapping(-1, 6), 5);
    assert_eq!(wrapping(0, 6), 0);
    assert_eq!(wrapping(3, 6), 3);
    assert_eq!(wrapping(6, 6), 0);

    assert_eq!(wrapping_index(0, 6), 0);
    assert_eq!(wrapping_index(3, 6), 3);
    assert_eq!(wrapping_index(6, 6), 0);
}

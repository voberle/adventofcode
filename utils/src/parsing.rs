/// Helping with parsing the input.

#[inline]
fn char(s: &str) -> char {
    s.chars().next().unwrap()
}

#[inline]
fn int<T>(s: &str) -> T
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    s.parse::<T>().unwrap()
}

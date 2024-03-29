use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IntChar<T>
where
    T: std::str::FromStr,
{
    Integer(T),
    Char(char),
}

impl<T> IntChar<T>
where
    T: std::str::FromStr,
{
    pub fn new(s: &str) -> Self {
        // if let Ok(val) = s.trim_start_matches('+').parse() {
        if let Ok(val) = s.trim().parse() {
            IntChar::Integer(val)
        } else if s.len() == 1 {
            IntChar::Char(s.chars().next().unwrap())
        } else {
            panic!("Invalid string for building IntChar: {}", s)
        }
    }

    pub fn get_integer(&self) -> &T {
        if let IntChar::Integer(i) = self {
            i
        } else {
            panic!("Wanted an integer")
        }
    }
}

impl<T> From<T> for IntChar<T>
where
    T: std::str::FromStr,
{
    fn from(item: T) -> Self {
        IntChar::Integer(item)
    }
}

impl<T> fmt::Display for IntChar<T>
where
    T: std::str::FromStr,
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Integer(v) => v.to_string(),
                Self::Char(v) => v.to_string(),
            }
        )
    }
}

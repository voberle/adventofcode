use std::fmt;

/// Mazes or pipes

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Pipe {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
}

// Such visualisation greatly helps.
impl fmt::Display for Pipe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Self::Vertical => '┃',
                Self::Horizontal => '━',
                Self::NorthEast => '┗',
                Self::NorthWest => '┛',
                Self::SouthWest => '┓',
                Self::SouthEast => '┏',
            }
        )
        // "-|F7LJ" => "━┃┏┓┗┛"
    }
}
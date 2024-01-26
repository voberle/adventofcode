use fxhash::FxHashMap;

use crate::intchar::IntChar;

#[derive(Debug)]
pub struct Registers<T> {
    regs: FxHashMap<char, T>,
}

impl<T> Registers<T>
where
    T: std::str::FromStr,
    T: Copy,
    T: Default,
{
    pub fn new() -> Self {
        Self {
            regs: FxHashMap::default(),
        }
    }

    pub fn get(&self, r: char) -> T {
        self.regs.get(&r).copied().unwrap_or_default()
    }

    pub fn set(&mut self, r: char, val: T) {
        self.regs.insert(r, val);
    }

    pub fn get_ic(&self, x: IntChar<T>) -> T {
        match x {
            IntChar::Integer(val) => val,
            IntChar::Char(src) => self.get(src),
        }
    }
}

impl<T> Default for Registers<T>
where
    T: std::str::FromStr,
    T: Copy,
    T: Default,
{
    fn default() -> Self {
        Self::new()
    }
}

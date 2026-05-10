use std::fmt;
use std::fmt::Formatter;

#[derive(PartialEq, Eq, Debug)]
pub enum Bit {
    Zero,
    One,
}

impl From<bool> for Bit {
    fn from(val: bool) -> Self {
        if val { Bit::One } else { Bit::Zero }
    }
}

impl fmt::Display for Bit {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let val = match self {
            Bit::Zero => "0",
            Bit::One => "1",
        };
        write!(f, "{}", val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_false() {
        let bit = Bit::from(false);
        assert_eq!(bit, Bit::Zero);
    }

    #[test]
    fn test_true() {
        let bit = Bit::from(true);
        assert_eq!(bit, Bit::One);
    }
}

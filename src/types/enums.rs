#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Strand {
    Forward,
    Reverse,
    #[default]
    Unknown,
}
impl FromStr for Strand {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Strand::Forward),
            "-" => Ok(Strand::Reverse),
            "." => Ok(Strand::Unknown),
            _ => Err("Strand must be either + or -"),
        }
    }
}
impl TryFrom<char> for Strand {
    type Error = &'static str;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '+' => Ok(Strand::Forward),
            '-' => Ok(Strand::Reverse),
            '.' => Ok(Strand::Unknown),
            _ => Err("Strand must be either + or -"),
        }
    }
}
impl TryFrom<u8> for Strand {
    type Error = &'static str;
    fn try_from(c: u8) -> Result<Self, Self::Error> {
        match c {
            b'+' => Ok(Strand::Forward),
            b'-' => Ok(Strand::Reverse),
            b'.' => Ok(Strand::Unknown),
            _ => Err("Strand must be either + or -"),
        }
    }
}
impl Into<char> for Strand {
    fn into(self) -> char {
        match self {
            Strand::Forward => '+',
            Strand::Reverse => '-',
            Strand::Unknown => '.',
        }
    }
}
impl Into<u8> for Strand {
    fn into(self) -> u8 {
        match self {
            Strand::Forward => b'+',
            Strand::Reverse => b'-',
            Strand::Unknown => b'.',
        }
    }
}
impl Display for Strand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Strand::Forward => write!(f, "+"),
            Strand::Reverse => write!(f, "-"),
            Strand::Unknown => write!(f, "."),
        }
    }
}

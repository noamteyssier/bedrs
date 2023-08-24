#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{
    cmp::Ordering,
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

    /// Convert a string to a Strand
    ///
    /// # Arguments
    /// * `s` - A string slice to convert to a Strand
    ///
    /// # Example
    ///
    /// ```
    /// use bedrs::Strand;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(Strand::from_str("+").unwrap(), Strand::Forward);
    /// assert_eq!(Strand::from_str("-").unwrap(), Strand::Reverse);
    /// assert_eq!(Strand::from_str(".").unwrap(), Strand::Unknown);
    /// assert!(Strand::from_str("a").is_err());
    /// ```
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

    /// Convert a char to a Strand
    ///
    /// # Arguments
    /// * `c` - A char to convert to a Strand
    ///
    /// # Example
    ///
    /// ```
    /// use bedrs::Strand;
    ///
    /// assert_eq!(Strand::try_from('+').unwrap(), Strand::Forward);
    /// assert_eq!(Strand::try_from('-').unwrap(), Strand::Reverse);
    /// assert_eq!(Strand::try_from('.').unwrap(), Strand::Unknown);
    /// assert!(Strand::try_from('a').is_err());
    /// ```
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

    /// Convert a u8 to a Strand
    ///
    /// # Arguments
    /// * `c` - A u8 to convert to a Strand
    ///
    /// # Example
    /// ```
    /// use bedrs::Strand;
    ///
    /// assert_eq!(Strand::try_from(b'+').unwrap(), Strand::Forward);
    /// assert_eq!(Strand::try_from(b'-').unwrap(), Strand::Reverse);
    /// assert_eq!(Strand::try_from(b'.').unwrap(), Strand::Unknown);
    /// assert!(Strand::try_from(b'a').is_err());
    /// ```
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
    /// Convert a Strand to a char
    ///
    /// # Example
    ///
    /// ```
    /// use bedrs::Strand;
    ///
    /// let char_forward: char = Strand::Forward.into();
    /// let char_reverse: char = Strand::Reverse.into();
    /// let char_unknown: char = Strand::Unknown.into();
    ///
    /// assert_eq!(char_forward, '+');
    /// assert_eq!(char_reverse, '-');
    /// assert_eq!(char_unknown, '.');
    /// ```
    fn into(self) -> char {
        match self {
            Strand::Forward => '+',
            Strand::Reverse => '-',
            Strand::Unknown => '.',
        }
    }
}
impl Into<u8> for Strand {
    /// Convert a Strand to a u8
    ///
    /// # Example
    ///
    /// ```
    /// use bedrs::Strand;
    ///
    /// let u8_forward: u8 = Strand::Forward.into();
    /// let u8_reverse: u8 = Strand::Reverse.into();
    /// let u8_unknown: u8 = Strand::Unknown.into();
    ///
    /// assert_eq!(u8_forward, b'+');
    /// assert_eq!(u8_reverse, b'-');
    /// assert_eq!(u8_unknown, b'.');
    /// ```
    fn into(self) -> u8 {
        match self {
            Strand::Forward => b'+',
            Strand::Reverse => b'-',
            Strand::Unknown => b'.',
        }
    }
}
impl Display for Strand {
    /// Display a strand in a human readable format
    ///
    /// - Forward is displayed as `+`
    /// - Reverse is displayed as `-`
    /// - Unknown is displayed as `.`
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Strand::Forward => write!(f, "+"),
            Strand::Reverse => write!(f, "-"),
            Strand::Unknown => write!(f, "."),
        }
    }
}
impl Ord for Strand {
    /// Sort order for Strand
    ///
    /// - Forward < Reverse < Unknown
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Strand::Forward, Strand::Forward) => Ordering::Equal,
            (Strand::Forward, Strand::Reverse) => Ordering::Less,
            (Strand::Forward, Strand::Unknown) => Ordering::Less,
            (Strand::Reverse, Strand::Forward) => Ordering::Greater,
            (Strand::Reverse, Strand::Reverse) => Ordering::Equal,
            (Strand::Reverse, Strand::Unknown) => Ordering::Less,
            (Strand::Unknown, Strand::Forward) => Ordering::Greater,
            (Strand::Unknown, Strand::Reverse) => Ordering::Greater,
            (Strand::Unknown, Strand::Unknown) => Ordering::Equal,
        }
    }
}
impl PartialOrd for Strand {
    /// Sort order for Strand
    ///
    /// - Forward < Reverse < Unknown
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod testing {

    use super::*;

    #[test]
    fn test_strand_from_str() {
        assert_eq!(Strand::from_str("+").unwrap(), Strand::Forward);
        assert_eq!(Strand::from_str("-").unwrap(), Strand::Reverse);
        assert_eq!(Strand::from_str(".").unwrap(), Strand::Unknown);
        assert!(Strand::from_str("a").is_err());
    }

    #[test]
    fn test_strand_try_from_char() {
        assert_eq!(Strand::try_from('+').unwrap(), Strand::Forward);
        assert_eq!(Strand::try_from('-').unwrap(), Strand::Reverse);
        assert_eq!(Strand::try_from('.').unwrap(), Strand::Unknown);
        assert!(Strand::try_from('a').is_err());
    }

    #[test]
    fn test_strand_try_from_u8() {
        assert_eq!(Strand::try_from(b'+').unwrap(), Strand::Forward);
        assert_eq!(Strand::try_from(b'-').unwrap(), Strand::Reverse);
        assert_eq!(Strand::try_from(b'.').unwrap(), Strand::Unknown);
        assert!(Strand::try_from(b'a').is_err());
    }

    #[test]
    fn test_strand_into_char() {
        let char_forward: char = Strand::Forward.into();
        let char_reverse: char = Strand::Reverse.into();
        let char_unknown: char = Strand::Unknown.into();
        assert_eq!(char_forward, '+');
        assert_eq!(char_reverse, '-');
        assert_eq!(char_unknown, '.');
    }

    #[test]
    fn test_strand_into_u8() {
        let u8_forward: u8 = Strand::Forward.into();
        let u8_reverse: u8 = Strand::Reverse.into();
        let u8_unknown: u8 = Strand::Unknown.into();
        assert_eq!(u8_forward, b'+');
        assert_eq!(u8_reverse, b'-');
        assert_eq!(u8_unknown, b'.');
    }

    #[test]
    fn test_strand_display() {
        assert_eq!(format!("{}", Strand::Forward), "+");
        assert_eq!(format!("{}", Strand::Reverse), "-");
        assert_eq!(format!("{}", Strand::Unknown), ".");
    }

    #[test]
    fn test_strand_default() {
        assert_eq!(Strand::default(), Strand::Unknown);
    }

    #[test]
    fn test_strand_eq() {
        assert_eq!(Strand::Forward, Strand::Forward);
        assert_eq!(Strand::Reverse, Strand::Reverse);
        assert_eq!(Strand::Unknown, Strand::Unknown);
    }

    #[test]
    fn test_strand_ne() {
        assert_ne!(Strand::Forward, Strand::Reverse);
        assert_ne!(Strand::Forward, Strand::Unknown);
        assert_ne!(Strand::Reverse, Strand::Unknown);
    }

    #[test]
    fn test_strand_clone() {
        assert_eq!(Strand::Forward.clone(), Strand::Forward);
        assert_eq!(Strand::Reverse.clone(), Strand::Reverse);
        assert_eq!(Strand::Unknown.clone(), Strand::Unknown);
    }

    #[test]
    fn test_strand_copy() {
        let strand = Strand::Forward;
        let strand_copy = strand;
        assert_eq!(strand, strand_copy);
    }

    #[test]
    fn test_strand_ordering() {
        assert!(Strand::Forward < Strand::Reverse);
        assert!(Strand::Forward < Strand::Unknown);
        assert!(Strand::Forward == Strand::Forward);

        assert!(Strand::Reverse > Strand::Forward);
        assert!(Strand::Reverse < Strand::Unknown);
        assert!(Strand::Reverse == Strand::Reverse);

        assert!(Strand::Unknown > Strand::Forward);
        assert!(Strand::Unknown > Strand::Reverse);
        assert!(Strand::Unknown == Strand::Unknown);
    }
}

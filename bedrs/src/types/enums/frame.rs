use std::{fmt::Display, str::FromStr};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Copy, Default, Hash, Eq, Ord, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Frame {
    #[cfg_attr(feature = "serde", serde(rename = "."))]
    #[default]
    None,
    #[cfg_attr(feature = "serde", serde(rename = "0"))]
    Zero,
    #[cfg_attr(feature = "serde", serde(rename = "1"))]
    One,
    #[cfg_attr(feature = "serde", serde(rename = "2"))]
    Two,
}

impl FromStr for Frame {
    type Err = &'static str;
    /// Convert a string to a Frame
    ///
    /// # Arguments
    /// * `s` - A string slice to convert to a Frame
    ///
    /// # Example
    ///
    /// ```
    /// use bedrs::Frame;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(Frame::from_str("0").unwrap(), Frame::Zero);
    /// assert_eq!(Frame::from_str("1").unwrap(), Frame::One);
    /// assert_eq!(Frame::from_str("2").unwrap(), Frame::Two);
    /// assert_eq!(Frame::from_str(".").unwrap(), Frame::None);
    /// assert!(Frame::from_str("a").is_err());
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Frame::Zero),
            "1" => Ok(Frame::One),
            "2" => Ok(Frame::Two),
            "." => Ok(Frame::None),
            _ => Err("Frame must be either 0, 1, 2 or ."),
        }
    }
}

impl From<char> for Frame {
    /// Convert a char to a Frame
    ///
    /// # Arguments
    /// * `c` - A char to convert to a Frame
    ///
    /// # Example
    ///
    /// ```
    /// use bedrs::Frame;
    /// use std::convert::TryFrom;
    ///
    /// assert_eq!(Frame::from('0'), Frame::Zero);
    /// assert_eq!(Frame::from('1'), Frame::One);
    /// assert_eq!(Frame::from('2'), Frame::Two);
    /// assert_eq!(Frame::from('.'), Frame::None);
    ///
    /// ```
    fn from(c: char) -> Self {
        match c {
            '0' => Frame::Zero,
            '1' => Frame::One,
            '2' => Frame::Two,
            _ => Frame::None,
        }
    }
}

impl From<i32> for Frame {
    /// Convert an i32 to a Frame
    ///
    /// # Arguments
    /// * `i` - An i32 to convert to a Frame
    ///
    /// # Example
    ///
    /// ```
    /// use bedrs::Frame;
    ///
    /// assert_eq!(Frame::from(0), Frame::Zero);
    /// assert_eq!(Frame::from(1), Frame::One);
    /// assert_eq!(Frame::from(2), Frame::Two);
    /// assert_eq!(Frame::from(3), Frame::None);
    ///
    /// ```
    fn from(i: i32) -> Self {
        match i {
            0 => Frame::Zero,
            1 => Frame::One,
            2 => Frame::Two,
            _ => Frame::None,
        }
    }
}

impl From<usize> for Frame {
    /// Convert a usize to a Frame
    ///
    /// # Arguments
    /// * `i` - A usize to convert to a Frame
    ///
    /// # Example
    ///
    /// ```
    /// use bedrs::Frame;
    ///
    /// assert_eq!(Frame::from(0), Frame::Zero);
    /// assert_eq!(Frame::from(1), Frame::One);
    /// assert_eq!(Frame::from(2), Frame::Two);
    /// assert_eq!(Frame::from(3), Frame::None);
    ///
    /// ```
    fn from(i: usize) -> Self {
        match i {
            0 => Frame::Zero,
            1 => Frame::One,
            2 => Frame::Two,
            _ => Frame::None,
        }
    }
}

impl From<Option<i32>> for Frame {
    /// Convert an `Option<i32>`to a Frame
    ///
    /// # Arguments
    /// * `i` - An `Option<i32>` to convert to a Frame
    ///
    /// # Example
    ///
    /// ```
    /// use bedrs::Frame;
    ///
    /// assert_eq!(Frame::from(Some(0)), Frame::Zero);
    /// assert_eq!(Frame::from(Some(1)), Frame::One);
    /// assert_eq!(Frame::from(Some(2)), Frame::Two);
    /// assert_eq!(Frame::from(None), Frame::None);
    ///
    /// ```
    fn from(i: Option<i32>) -> Self {
        match i {
            Some(0) => Frame::Zero,
            Some(1) => Frame::One,
            Some(2) => Frame::Two,
            _ => Frame::None,
        }
    }
}

impl Display for Frame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Frame::Zero => write!(f, "0"),
            Frame::One => write!(f, "1"),
            Frame::Two => write!(f, "2"),
            Frame::None => write!(f, "."),
        }
    }
}

#[cfg(test)]
mod testing {
    use super::*;

    #[test]
    fn test_frame() {
        assert_eq!(Frame::Zero, Frame::Zero);
        assert_eq!(Frame::One, Frame::One);
        assert_eq!(Frame::Two, Frame::Two);
        assert_eq!(Frame::None, Frame::None);
    }

    #[test]
    fn test_frame_default() {
        assert_eq!(Frame::default(), Frame::None);
    }

    #[test]
    fn test_frame_from_str() {
        assert_eq!(Frame::from_str("0").unwrap(), Frame::Zero);
        assert_eq!(Frame::from_str("1").unwrap(), Frame::One);
        assert_eq!(Frame::from_str("2").unwrap(), Frame::Two);
        assert_eq!(Frame::from_str(".").unwrap(), Frame::None);
        assert!(Frame::from_str("a").is_err());
    }

    #[test]
    fn test_frame_from_char() {
        assert_eq!(Frame::from('0'), Frame::Zero);
        assert_eq!(Frame::from('1'), Frame::One);
        assert_eq!(Frame::from('2'), Frame::Two);
        assert_eq!(Frame::from('.'), Frame::None);
        assert_eq!(Frame::from('a'), Frame::None);
    }

    #[test]
    fn test_frame_from_i32() {
        assert_eq!(Frame::from(0), Frame::Zero);
        assert_eq!(Frame::from(1), Frame::One);
        assert_eq!(Frame::from(2), Frame::Two);
        assert_eq!(Frame::from(3), Frame::None);
    }

    #[test]
    fn test_frame_from_usize() {
        assert_eq!(Frame::from(0), Frame::Zero);
        assert_eq!(Frame::from(1), Frame::One);
        assert_eq!(Frame::from(2), Frame::Two);
        assert_eq!(Frame::from(3), Frame::None);
    }

    #[test]
    fn test_frame_from_option_i32() {
        assert_eq!(Frame::from(Some(0)), Frame::Zero);
        assert_eq!(Frame::from(Some(1)), Frame::One);
        assert_eq!(Frame::from(Some(2)), Frame::Two);
        assert_eq!(Frame::from(None), Frame::None);
    }

    #[test]
    fn test_frame_ordering() {
        assert!(Frame::None < Frame::Zero);
        assert!(Frame::Zero < Frame::One);
        assert!(Frame::One < Frame::Two);
    }
}

#[cfg(feature = "serde")]
#[cfg(test)]
mod serde_testing {
    use super::*;
    use csv::ReaderBuilder;

    #[test]
    fn test_csv_deserialization() {
        let a = "0\n1\n2\n.\n";
        let mut rdr = ReaderBuilder::new()
            .has_headers(false)
            .from_reader(a.as_bytes());
        let mut iter = rdr.deserialize();
        let f1: Frame = iter.next().unwrap().unwrap();
        let f2: Frame = iter.next().unwrap().unwrap();
        let f3: Frame = iter.next().unwrap().unwrap();
        let f4: Frame = iter.next().unwrap().unwrap();
        assert!(iter.next().is_none());
        assert_eq!(f1, Frame::Zero);
        assert_eq!(f2, Frame::One);
        assert_eq!(f3, Frame::Two);
        assert_eq!(f4, Frame::None);
    }

    #[test]
    fn test_csv_serialization() {
        let records = vec![Frame::Zero, Frame::One, Frame::Two, Frame::None];
        let mut wtr = csv::Writer::from_writer(vec![]);
        for r in records {
            wtr.serialize(r).unwrap();
        }
        let result = String::from_utf8(wtr.into_inner().unwrap()).unwrap();
        assert_eq!(result, "0\n1\n2\n.\n");
    }
}

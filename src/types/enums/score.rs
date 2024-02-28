use std::{fmt::Display, str::FromStr};

#[cfg(feature = "serde")]
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
#[cfg(feature = "serde")]
use std::fmt;

#[derive(Debug, PartialEq, Clone, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Score(
    #[cfg_attr(
        feature = "serde",
        serde(
            deserialize_with = "deserialize_option_float",
            serialize_with = "serialize_option_float",
        )
    )]
    pub Option<f64>,
);

// Custom deserializer that expects a string and parses it as f64 or interprets '.' as None
#[cfg(feature = "serde")]
fn deserialize_option_float<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
where
    D: Deserializer<'de>,
{
    struct OptionFloatVisitor;

    impl<'de> de::Visitor<'de> for OptionFloatVisitor {
        type Value = Option<f64>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a string representing a float or '.'")
        }

        // Directly visit_str to interpret the string
        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            if value == "." {
                Ok(None)
            } else {
                match f64::from_str(value) {
                    Ok(val) => Ok(Some(val)),
                    Err(_) => Err(E::custom(format!("failed to parse float from {value}"))),
                }
            }
        }
    }

    deserializer.deserialize_str(OptionFloatVisitor)
}

#[cfg(feature = "serde")]
// Custom serializer to convert Option<f64> to string, writing None as '.'
fn serialize_option_float<S>(option: &Option<f64>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match *option {
        Some(value) => serializer.serialize_some(&value.to_string()),
        None => serializer.serialize_str("."),
    }
}
impl Display for Score {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(val) = self.0 {
            write!(f, "{val}")
        } else {
            write!(f, ".")
        }
    }
}
impl FromStr for Score {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "." => Ok(Score(None)),
            _ => match s.parse::<f64>() {
                Ok(val) => Ok(Score(Some(val))),
                Err(_) => Err("Could not parse score"),
            },
        }
    }
}
impl From<f64> for Score {
    fn from(s: f64) -> Self {
        Score(Some(s))
    }
}
#[allow(clippy::cast_lossless)]
impl From<f32> for Score {
    fn from(s: f32) -> Self {
        Score(Some(s as f64))
    }
}
#[allow(clippy::cast_lossless)]
impl From<i32> for Score {
    fn from(s: i32) -> Self {
        Score(Some(s as f64))
    }
}
#[allow(clippy::cast_precision_loss)]
impl From<usize> for Score {
    fn from(s: usize) -> Self {
        Score(Some(s as f64))
    }
}
impl From<Option<f64>> for Score {
    fn from(s: Option<f64>) -> Self {
        Score(s)
    }
}

#[cfg(test)]
mod testing {

    use super::*;
    use std::str::FromStr;
    #[test]
    fn test_score_display() {
        let a = Score(Some(10.0));
        assert_eq!(a.to_string(), "10");
        let b = Score(None);
        assert_eq!(b.to_string(), ".");
        let c = Score(Some(11.1));
        assert_eq!(c.to_string(), "11.1");
    }
    #[test]
    fn test_score_from_str() {
        let a = Score::from_str("10.0").unwrap();
        assert_eq!(a, Score(Some(10.0)));
        let b = Score::from_str(".").unwrap();
        assert_eq!(b, Score(None));
        let c = Score::from_str("10").unwrap();
        assert_eq!(c, Score(Some(10.0)));
    }
    #[test]
    fn test_score_from_f64() {
        let a = Score::from(10.0);
        assert_eq!(a, Score(Some(10.0)));
    }
    #[test]
    fn test_score_from_f32() {
        let a = Score::from(10.0);
        assert_eq!(a, Score(Some(10.0)));
    }
    #[test]
    fn test_score_from_i32() {
        let a = Score::from(10);
        assert_eq!(a, Score(Some(10.0)));
    }
    #[test]
    fn test_score_from_usize() {
        let a = Score::from(10);
        assert_eq!(a, Score(Some(10.0)));
    }
    #[test]
    fn test_score_from_option_f64() {
        let a = Score::from(Some(10.0));
        assert_eq!(a, Score(Some(10.0)));
        let b = Score::from(None);
        assert_eq!(b, Score(None));
    }
    #[test]
    fn test_score_from_str_fail() {
        let a = Score::from_str("a");
        assert!(a.is_err());
    }
}

#[cfg(feature = "serde")]
#[cfg(test)]
mod serde_testing {
    use super::*;
    use csv::ReaderBuilder;

    #[test]
    fn test_csv_deserialization() {
        let a = "10\n.\n11.1\n";
        let rdr = ReaderBuilder::new()
            .has_headers(false)
            .from_reader(a.as_bytes());
        let mut iter = rdr.into_deserialize();
        let r1: Score = iter.next().unwrap().unwrap();
        let r2: Score = iter.next().unwrap().unwrap();
        let r3: Score = iter.next().unwrap().unwrap();
        assert_eq!(r1, Score(Some(10.0)));
        assert_eq!(r2, Score(None));
        assert_eq!(r3, Score(Some(11.1)));
    }

    #[test]
    fn test_csv_serialization() {
        let a = vec![Score(Some(10.0)), Score(None), Score(Some(11.1))];
        let mut wtr = csv::Writer::from_writer(vec![]);
        for record in a {
            wtr.serialize(record).unwrap();
        }
        let result = String::from_utf8(wtr.into_inner().unwrap()).unwrap();
        assert_eq!(result, "10\n.\n11.1\n");
    }
}

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
        Score::Score(s)
    }
}
#[allow(clippy::cast_lossless)]
impl From<f32> for Score {
    fn from(s: f32) -> Self {
        Score::Score(s as f64)
    }
}
#[allow(clippy::cast_lossless)]
impl From<i32> for Score {
    fn from(s: i32) -> Self {
        Score::Score(s as f64)
    }
}
#[allow(clippy::cast_precision_loss)]
impl From<usize> for Score {
    fn from(s: usize) -> Self {
        Score::Score(s as f64)
    }
}
impl From<Option<f64>> for Score {
    fn from(s: Option<f64>) -> Self {
        match s {
            Some(val) => Score::Score(val),
            None => Score::Empty,
        }
    }
}
#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for Score {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Option::deserialize(deserializer).map(Into::into)
    }
}

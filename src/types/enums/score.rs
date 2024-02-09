#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize};
use std::{fmt::Display, str::FromStr};

#[derive(Debug, PartialEq, Clone, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum Score {
    Score(f64),
    #[cfg_attr(feature = "serde", serde(rename = "."))]
    #[default]
    Empty,
}
impl Display for Score {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Score::Score(s) => write!(f, "{s}"),
            Score::Empty => write!(f, "."),
        }
    }
}
impl FromStr for Score {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "." => Ok(Score::Empty),
            _ => match s.parse::<f64>() {
                Ok(val) => Ok(Score::Score(val)),
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

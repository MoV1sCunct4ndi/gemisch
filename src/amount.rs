use std::fmt::{Display, Formatter};
use std::ops::Add;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Amount {
    pub value: Range<f32>,
    pub unit: MeasurementUnit,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MeasurementUnit {
    None,
    Mass,
    Volume,
    Other(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Range<T> {
    lower: T,
    upper: T,
}

impl<T> Add for Range<T> where T: Add<Output=T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            lower: self.lower + rhs.lower,
            upper: self.upper + rhs.upper,
        }
    }
}

impl<T> From<T> for Range<T> where T: Copy {
    fn from(value: T) -> Self {
        Self {
            lower: value,
            upper: value,
        }
    }
}

impl<T> Display for Range<T> where T: Display + PartialEq {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.lower == self.upper {
            write!(f, "{}", self.lower)
        } else {
            write!(f, "{}-{}", self.lower, self.upper)
        }
    }
}
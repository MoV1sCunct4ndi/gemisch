use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::ops::{Add, Mul};
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

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
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

impl<T> Mul<T> for Range<T> where T: Mul<Output=T> +Copy {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self{
            lower: self.lower * rhs,
            upper: self.upper * rhs,
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

impl<T> PartialEq<T> for Range<T> where T: PartialEq {
    fn eq(&self, other: &T) -> bool {
        self.lower == self.upper && self.lower == *other
    }
}

impl<T> PartialOrd<T> for Range<T> where T: PartialOrd {
    fn partial_cmp(&self, other: &T) -> Option<Ordering> {
        if self == other {
            return Some(Ordering::Equal);
        }
        if self.upper < *other {
            return Some(Ordering::Less);
        }
        if self.lower > *other {
            return Some(Ordering::Greater);
        }
        None
    }
}

impl Display for Amount {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.unit {
            MeasurementUnit::None => write!(f, "{}", self.value),
            MeasurementUnit::Mass => {
                if self.value < 1.0 {
                    write!(f, "{}g", self.value * 1000.0)
                } else {
                    write!(f, "{}kg", self.value)
                }
            }
            MeasurementUnit::Volume => {
                if self.value < 0.01 {
                    write!(f, "{}ml", self.value * 1000.0)
                } else if self.value < 0.1 {
                    write!(f, "{}cl", self.value * 100.0)
                } else {
                    write!(f, "{}l", self.value)
                }
            }
            MeasurementUnit::Other(s) => write!(f, "{}{}", self.value, s),
        }
    }
}

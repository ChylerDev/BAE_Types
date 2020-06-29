//! # BAE_Types
//!
//! The common types used throughout the Broad Audio Engine.

#![warn(missing_docs)]
#![doc(html_root_url = "https://docs.rs/bae_types/0.14.2")]

use std::cmp::Ordering;

/// Type used for mathematical calculations that need to be fast (e.g. sample calculation).
pub type FastMath = f32;

/// Type used for mathematical calculations that need to be accurate (e.g. some forms of fine-tuned parameter control).
pub type AccurateMath = f64;

/// Type to perform infrequent mathematical operations with when accuracy is needed.
#[derive(Debug, Default, Copy, Clone)]
pub struct Math(pub AccurateMath);

impl From<AccurateMath> for Math {
    fn from(x: AccurateMath) -> Self {
        Math(x)
    }
}
impl Into<AccurateMath> for Math {
    fn into(self) -> AccurateMath {
        self.0
    }
}
impl PartialEq for Math {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl PartialOrd for Math {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.0.is_nan() || other.0.is_nan() {
            None
        } else if self.0 < other.0 {
            Some(Ordering::Less)
        } else if self.0 > other.0 {
            Some(Ordering::Greater)
        } else {
            Some(Ordering::Equal)
        }
    }
}

/// Type to calculate samples with.
#[derive(Debug, Default, Copy, Clone)]
pub struct Sample(pub FastMath);

impl From<FastMath> for Sample {
    fn from(x: FastMath) -> Self {
        Sample(x)
    }
}
impl Into<FastMath> for Sample {
    fn into(self) -> FastMath {
        self.0
    }
}
impl PartialEq for Sample {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl PartialOrd for Sample {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.0.is_nan() || other.0.is_nan() {
            None
        } else if self.0 < other.0 {
            Some(Ordering::Less)
        } else if self.0 > other.0 {
            Some(Ordering::Greater)
        } else {
            Some(Ordering::Equal)
        }
    }
}

/// Shorthand for a vector containing sample data.
pub type SampleTrack = Vec<Sample>;

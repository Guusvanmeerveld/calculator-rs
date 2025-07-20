use std::{num::ParseIntError, str::FromStr};

use derive_more::{Add, Display, Div, Mul, Neg, Sub};

use crate::ast::literal::{Number, float::Float};

#[derive(Debug, PartialEq, Add, Mul, Neg, Div, Sub, Display)]
#[mul(forward)]
#[div(forward)]
pub struct Int(isize);

impl Number for Int {
    fn as_f64(&self) -> f64 {
        self.0 as f64
    }
    fn as_u32(&self) -> u32 {
        self.0 as u32
    }
}

impl Int {
    pub fn new(int: isize) -> Self {
        Self(int)
    }

    pub fn pow<N: super::Number>(&self, exp: N) -> Self {
        Self(self.0.pow(exp.as_u32()))
    }
}

impl FromStr for Int {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let int: isize = s.parse()?;

        Ok(Self(int))
    }
}

impl Into<Float> for Int {
    fn into(self) -> Float {
        Float::new(self.0 as f64)
    }
}

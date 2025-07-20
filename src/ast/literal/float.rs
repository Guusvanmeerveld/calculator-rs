use std::{fmt::Display, num::ParseFloatError, str::FromStr};

use derive_more::{Add, Div, Mul, Neg, Sub};

use crate::ast::literal::{Number, int::Int};

#[derive(Debug, PartialEq, Add, Mul, Neg, Div, Sub)]
#[mul(forward)]
#[div(forward)]
pub struct Float(f64);

impl Number for Float {
    fn as_f64(&self) -> f64 {
        self.0
    }
    fn as_u32(&self) -> u32 {
        self.0 as u32
    }
}

impl Display for Float {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Float {
    pub fn new(float: f64) -> Self {
        Self(float)
    }

    pub fn pow<N: super::Number>(&self, exp: N) -> Self {
        Self(self.0.powf(exp.as_f64()))
    }
}

impl FromStr for Float {
    type Err = ParseFloatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let float: f64 = s.parse()?;

        Ok(Self(float))
    }
}

impl Into<Int> for Float {
    fn into(self) -> Int {
        Int::new(self.0 as isize)
    }
}

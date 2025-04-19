use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Neg, Sub},
};

// use crate::error::Result;

#[derive(Debug, PartialEq)]
pub enum Literal {
    // String(String),
    Number(isize),
    Float(f64),
}

impl Neg for Literal {
    type Output = Literal;

    fn neg(self) -> Self::Output {
        match self {
            Literal::Float(value) => Literal::Float(value.neg()),
            Literal::Number(value) => Literal::Number(value.neg()),
        }
    }
}

impl Literal {
    pub fn pow(self, rhs: Self) -> Self {
        match self {
            Literal::Float(lhs_value) => match rhs {
                Literal::Float(rhs_value) => Literal::Float(lhs_value.powf(rhs_value)),
                Literal::Number(rhs_value) => Literal::Float(lhs_value.powf(rhs_value as f64)),
            },
            Literal::Number(lhs_value) => match rhs {
                Literal::Float(rhs_value) => Literal::Float((lhs_value as f64).powf(rhs_value)),
                Literal::Number(rhs_value) => Literal::Number(lhs_value.pow(rhs_value as u32)),
            },
        }
    }
}

impl Div for Literal {
    type Output = Literal;

    fn div(self, rhs: Self) -> Self::Output {
        match self {
            Literal::Float(lhs_value) => match rhs {
                Literal::Float(rhs_value) => Literal::Float(lhs_value / rhs_value),
                Literal::Number(rhs_value) => Literal::Float(lhs_value / (rhs_value as f64)),
            },
            Literal::Number(lhs_value) => match rhs {
                Literal::Float(rhs_value) => Literal::Float((lhs_value as f64) / rhs_value),
                Literal::Number(rhs_value) => Literal::Float((lhs_value / rhs_value) as f64),
            },
        }
    }
}

impl Mul for Literal {
    type Output = Literal;

    fn mul(self, rhs: Self) -> Self::Output {
        match self {
            Literal::Float(lhs_value) => match rhs {
                Literal::Float(rhs_value) => Literal::Float(lhs_value * rhs_value),
                Literal::Number(rhs_value) => Literal::Float(lhs_value * (rhs_value as f64)),
            },
            Literal::Number(lhs_value) => match rhs {
                Literal::Float(rhs_value) => Literal::Float((lhs_value as f64) * rhs_value),
                Literal::Number(rhs_value) => Literal::Number(lhs_value * rhs_value),
            },
        }
    }
}

impl Sub for Literal {
    type Output = Literal;

    fn sub(self, rhs: Self) -> Self::Output {
        match self {
            Literal::Float(lhs_value) => match rhs {
                Literal::Float(rhs_value) => Literal::Float(lhs_value - rhs_value),
                Literal::Number(rhs_value) => Literal::Float(lhs_value - (rhs_value as f64)),
            },
            Literal::Number(lhs_value) => match rhs {
                Literal::Float(rhs_value) => Literal::Float((lhs_value as f64) - rhs_value),
                Literal::Number(rhs_value) => Literal::Number(lhs_value - rhs_value),
            },
        }
    }
}

impl Add for Literal {
    type Output = Literal;

    fn add(self, rhs: Self) -> Self::Output {
        match self {
            Literal::Float(lhs_value) => match rhs {
                Literal::Float(rhs_value) => Literal::Float(lhs_value + rhs_value),
                Literal::Number(rhs_value) => Literal::Float(lhs_value + (rhs_value as f64)),
            },
            Literal::Number(lhs_value) => match rhs {
                Literal::Float(rhs_value) => Literal::Float((lhs_value as f64) + rhs_value),
                Literal::Number(rhs_value) => Literal::Number(lhs_value + rhs_value),
            },
        }
    }
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Number(number) => number.to_string(),
                Self::Float(float) => float.to_string(),
            }
        )
    }
}

use std::ops;

use crate::ast::literal::Float;
use crate::ast::{BinaryOperator, Expression, Literal, UnaryOperator};
use crate::error::Result;

use super::Interpreter;

pub struct SimpleInterpreter {}

impl Interpreter for SimpleInterpreter {
    fn eval(expression: Expression) -> Result<Literal> {
        match expression {
            Expression::Literal(literal) => Ok(literal),
            Expression::BinaryExpr { op, lhs, rhs } => {
                let lhs_output = Self::eval(*lhs)?;
                let rhs_output = Self::eval(*rhs)?;

                match op {
                    BinaryOperator::Add => Ok(lhs_output + rhs_output),
                    BinaryOperator::Subtract => Ok(lhs_output - rhs_output),
                    BinaryOperator::Divide => Ok(lhs_output / rhs_output),
                    BinaryOperator::Multiply => Ok(lhs_output * rhs_output),
                    BinaryOperator::Power => Ok(lhs_output.pow(rhs_output)),
                }
            }
            Expression::UnaryExpr { op, child } => {
                let child_output = Self::eval(*child)?;

                match op {
                    UnaryOperator::Negation => Ok(-child_output),
                    UnaryOperator::Parenthesis => Ok(child_output),
                }
            }
        }
    }
}

impl ops::Neg for Literal {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Self::Float(value) => Literal::Float(value.neg()),
            Self::Int(value) => Literal::Int(value.neg()),
        }
    }
}

impl Literal {
    pub fn pow(self, rhs: Self) -> Self {
        match self {
            Literal::Float(lhs_value) => match rhs {
                Literal::Float(rhs_value) => Literal::Float(lhs_value.pow(rhs_value)),
                Literal::Int(rhs_value) => Literal::Float(lhs_value.pow(rhs_value)),
            },
            Literal::Int(lhs_value) => match rhs {
                Literal::Float(rhs_value) => Literal::Int((lhs_value).pow(rhs_value)),
                Literal::Int(rhs_value) => Literal::Int((lhs_value).pow(rhs_value)),
            },
        }
    }
}

macro_rules! impl_op {
    ($op_name:ident, $func_name:ident) => {
        impl ops::$op_name for Literal {
            type Output = Self;

            fn $func_name(self, rhs: Self) -> Self::Output {
                match self {
                    Literal::Float(lhs_value) => match rhs {
                        Literal::Float(rhs_value) => Self::Float(lhs_value.$func_name(rhs_value)),
                        Literal::Int(rhs_value) => {
                            Self::Float(lhs_value.$func_name(Into::<Float>::into(rhs_value)))
                        }
                    },
                    Literal::Int(lhs_value) => match rhs {
                        Literal::Float(rhs_value) => {
                            Self::Float(Into::<Float>::into(lhs_value).$func_name(rhs_value))
                        }
                        Literal::Int(rhs_value) => Self::Int(lhs_value.$func_name(rhs_value)),
                    },
                }
            }
        }
    };
}

impl_op!(Div, div);
impl_op!(Mul, mul);
impl_op!(Add, add);
impl_op!(Sub, sub);

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

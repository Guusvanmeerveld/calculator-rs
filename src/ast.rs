use std::fmt::Display;

use crate::{
    error::Error,
    lexer::{Literal, Token},
};

#[derive(Debug)]
pub enum Expression {
    UnaryExpr {
        op: UnaryOperator,
        child: Box<Expression>,
    },
    BinaryExpr {
        op: BinaryOperator,
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    Literal(Literal),
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Literal(literal) => {
                write!(f, "{}", literal.to_string())
            }
            Self::BinaryExpr { op, lhs, rhs } => {
                let op_string = match op {
                    BinaryOperator::Add => "+",
                    BinaryOperator::Subtract => "^",
                    BinaryOperator::Divide => "/",
                    BinaryOperator::Multiply => "*",
                    BinaryOperator::Power => "^",
                };

                write!(f, "{} {} {}", lhs, op_string, rhs)
            }
            Self::UnaryExpr { op, child } => match op {
                UnaryOperator::Parenthesis => write!(f, "({})", child),
                UnaryOperator::Negation => write!(f, "-{}", child),
            },
        }
    }
}

#[derive(Debug)]
pub enum UnaryOperator {
    Parenthesis,
    Negation,
}

#[derive(Debug)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Divide,
    Multiply,
    Power,
}

impl TryFrom<Token> for BinaryOperator {
    type Error = Error;

    fn try_from(value: Token) -> std::result::Result<Self, Self::Error> {
        match value {
            Token::Plus => Ok(BinaryOperator::Add),
            Token::Dash => Ok(BinaryOperator::Subtract),
            Token::Star => Ok(BinaryOperator::Multiply),
            Token::ForwardSlash => Ok(BinaryOperator::Divide),
            Token::Hat => Ok(BinaryOperator::Power),
            _ => {
                // Handle non binary operator token
                todo!()
            }
        }
    }
}

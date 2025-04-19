use std::io::Read;
use std::iter::Peekable;

use crate::error::{Error, Result};
use crate::lexer::{Lexer, Literal, Token};

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

/// Grammar:
///
/// E → E + T
///
/// E → E - T
///
/// E → T
///
/// T → T * F
///
/// T → T / F
///
/// T → F
///
/// F → ( E )
///
/// F → - F
///
/// F → n
pub struct Parser {}

impl Parser {
    fn factor<R: Read>(lexer: &mut Peekable<Lexer<R>>) -> Result<Expression> {
        match lexer.next() {
            Some(token) => match token {
                Token::Literal(literal) => return Ok(Expression::Literal(literal)),
                Token::LeftParenthesis => {
                    let child = Self::expr(lexer)?;

                    match lexer.next() {
                        Some(token) => {
                            if token == Token::RightParenthesis {
                                return Ok(Expression::UnaryExpr {
                                    op: UnaryOperator::Parenthesis,
                                    child: Box::new(child),
                                });
                            } else {
                                // Handle missing closing parenthesis
                                todo!()
                            }
                        }
                        None => {
                            // Handle missing closing parenthesis
                            todo!()
                        }
                    }
                }
                Token::Dash => {
                    let child = Self::factor(lexer)?;

                    return Ok(Expression::UnaryExpr {
                        op: UnaryOperator::Negation,
                        child: Box::new(child),
                    });
                }
                _ => {
                    // Handle other tokens
                    todo!()
                }
            },
            None => {
                // Handle missing factor
                todo!()
            }
        }
    }

    fn term<R: Read>(lexer: &mut Peekable<Lexer<R>>) -> Result<Expression> {
        let lhs = Self::factor(lexer)?;

        while let Some(operator_token) = lexer.next_if(Token::is_term) {
            let rhs = Self::term(lexer)?;

            return Ok(Expression::BinaryExpr {
                op: operator_token.try_into()?,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            });
        }

        return Ok(lhs);
    }

    fn expr<R: Read>(lexer: &mut Peekable<Lexer<R>>) -> Result<Expression> {
        let lhs = Self::term(lexer)?;

        while let Some(operator_token) = lexer.next_if(Token::is_expression) {
            let rhs = Self::expr(lexer)?;

            return Ok(Expression::BinaryExpr {
                op: operator_token.try_into()?,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            });
        }

        return Ok(lhs);
    }

    pub fn parse<R: Read>(lexer: Lexer<R>) -> Result<Expression> {
        Self::expr(&mut lexer.peekable())
    }
}

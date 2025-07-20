use std::io::Read;
use std::iter::Peekable;

use crate::ast::{Expression, UnaryOperator};
use crate::error::{Error, Result, SyntaxError};
use crate::lexer::{Lexer, Token};

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
pub struct Parser;

impl Parser {
    fn factor<R: Read>(lexer: &mut Peekable<&mut Lexer<R>>) -> Result<Expression> {
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
                                todo!("Handle missing closing parenthesis")
                            }
                        }
                        None => {
                            todo!("Handle missing closing parenthesis")
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
                Token::Unrecognized(char) => {
                    Err(Error::SyntaxError(SyntaxError::UnrecognizedToken(char)))
                }

                token => {
                    todo!("Handle other tokens: {}", token)
                }
            },
            None => {
                todo!("Handle missing factor")
            }
        }
    }

    fn term<R: Read>(lexer: &mut Peekable<&mut Lexer<R>>) -> Result<Expression> {
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

    fn expr<R: Read>(lexer: &mut Peekable<&mut Lexer<R>>) -> Result<Expression> {
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

    pub fn parse<R: Read>(lexer: &mut Lexer<R>) -> Result<Expression> {
        Self::expr(&mut lexer.peekable())
    }
}

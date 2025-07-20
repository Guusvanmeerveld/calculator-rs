use crate::{
    ast::Literal,
    error::{Error, Result, SyntaxError},
};

use std::{fmt::Display, io::Read, iter::Peekable};

#[derive(Debug, PartialEq)]
pub enum Token {
    Plus,
    Dash,
    Star,
    ForwardSlash,
    Hat,
    LeftParenthesis,
    RightParenthesis,
    Literal(Literal),
    // Identifier(Identifier),
    Unrecognized(char),
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match self {
            Token::Plus => "+",
            Token::Dash => "-",
            Token::Star => "*",
            Token::ForwardSlash => "/",
            Token::Hat => "^",
            Token::LeftParenthesis => "(",
            Token::RightParenthesis => ")",
            Token::Literal(literal) => &literal.to_string(),
            Token::Unrecognized(char) => &char.to_string(),
        };

        write!(f, "{}", output)
    }
}

impl Token {
    pub fn is_expression(&self) -> bool {
        matches!(self, Token::Dash | Token::Plus)
    }

    pub fn is_term(&self) -> bool {
        matches!(self, Token::Star | Token::ForwardSlash | Token::Hat)
    }
}

// #[derive(Debug, PartialEq)]
// pub enum Identifier {}

const BLOCK_SIZE: usize = 512;

struct Buffer<S: Read> {
    stream: S,
    inner: Vec<u8>,
    position: usize,
}

impl<S: Read> Buffer<S> {
    fn new(stream: S) -> Self {
        Self {
            stream,
            inner: vec![0; BLOCK_SIZE],
            position: 0,
        }
    }

    fn read_block(&mut self) -> Result<usize> {
        if self.inner.len() != BLOCK_SIZE {
            self.inner.resize(BLOCK_SIZE, 0);
        }

        let byte_count = self.stream.read(&mut self.inner)?;

        // println!("Bytes read: {}", byte_count);

        // Remove the part of the vec that is not used.
        if byte_count < BLOCK_SIZE {
            self.inner.truncate(byte_count);
        }

        Ok(byte_count)
    }
}

impl<S: Read> Iterator for Buffer<S> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        // If we are at the 0 position, get some data.
        if self.position == 0 {
            if self.read_block().expect("Failed to read data from stream") == 0 {
                // Stop if we don't read any new data.
                return None;
            };
        }

        let char = self.inner[self.position];

        // If current position is going out of vector, we reset.
        if self.position == (self.inner.len() - 1) {
            self.position = 0;
        } else {
            self.position += 1;
        }

        Some(char)
    }
}

/// Reads the input and tries to tokenize it.
pub struct Lexer<S: Read> {
    buf: Peekable<Buffer<S>>,
    errors: Vec<Error>,
}

impl<S: Read> Lexer<S> {
    pub fn new(input: S) -> Self {
        Lexer {
            buf: Buffer::new(input).peekable(),
            errors: Vec::new(),
        }
    }

    /// Check if item is a digit or a "." (For example if it is a float)
    fn is_digit(char: &u8) -> bool {
        char.is_ascii_digit() || *char == b'.'
    }

    fn read_number_literal(&mut self, char: u8) -> Option<Literal> {
        if !Self::is_digit(&char) {
            return None;
        };

        let mut literal_as_ascii: Vec<u8> = Vec::new();

        // Add our first character.
        literal_as_ascii.push(char);

        // Only if the next bytes are also digits, we advance.
        while let Some(digit) = self.buf.next_if(Self::is_digit) {
            literal_as_ascii.push(digit);
        }

        // If it contains a '.' but no digits then this is not a number literal.
        if literal_as_ascii.contains(&b'.')
            && !literal_as_ascii.iter().any(|char| char.is_ascii_digit())
        {
            return None;
        }

        let seperator_count = literal_as_ascii.iter().filter(|x| **x == b'.').count();

        let literal_as_string = String::from_utf8(literal_as_ascii).ok()?;

        // If there are multiple '.', this is not a valid float
        if seperator_count > 1 {
            return None;
        } else if seperator_count == 1 {
            // Parse to float value.
            match literal_as_string.parse() {
                Ok(float_value) => Some(Literal::Float(float_value)),
                Err(err) => {
                    self.errors
                        .push(Error::SyntaxError(SyntaxError::ParseFloat(err)));

                    None
                }
            }
        } else {
            // Parse to number value.
            match literal_as_string.parse() {
                Ok(number_value) => Some(Literal::Int(number_value)),
                Err(err) => {
                    self.errors
                        .push(Error::SyntaxError(SyntaxError::ParseInt(err)));

                    None
                }
            }
        }
    }

    pub fn errors(&self) -> &[Error] {
        &self.errors
    }
}

impl<R: Read> Iterator for Lexer<R> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        // Reads the next character of the buffer.
        match self.buf.next()? {
            b'+' => Some(Token::Plus),
            b'-' => Some(Token::Dash),
            b'*' => Some(Token::Star),
            b'/' => Some(Token::ForwardSlash),
            b'^' => Some(Token::Hat),
            b'(' => Some(Token::LeftParenthesis),
            b')' => Some(Token::RightParenthesis),
            char => {
                if char.is_ascii_whitespace() {
                    // Skip whitespace
                    return self.next();
                }

                if let Some(literal) = self.read_number_literal(char) {
                    return Some(Token::Literal(literal));
                }

                // self.errors
                //     .push(Error::SyntaxError(SyntaxError::UnrecognizedToken(
                //         char::from_u32(char as u32).unwrap_or('0'),
                //     )));

                return Some(Token::Unrecognized(
                    char::from_u32(char as u32).unwrap_or('0'),
                ));
                // if self.is_letter() {}
            }
        }
    }
}

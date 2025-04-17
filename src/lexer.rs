use crate::error::Result;
use std::{io::Read, iter::Peekable};

#[derive(Debug, PartialEq)]
pub enum Token {
    Plus,
    Minus,
    Multiply,
    Divide,
    Power,
    LeftParenthesis,
    RightParenthesis,
    Literal(Literal),
    // Identifier(Identifier),
    Whitespace,
    Unrecognized,
}

#[derive(Debug, PartialEq)]
pub enum Literal {
    // String(String),
    Number(isize),
    Float(f64),
}

// #[derive(Debug, PartialEq)]
// pub enum Identifier {}

const BLOCK_SIZE: usize = 512;

struct Buffer<S: Read> {
    stream: S,
    inner: Vec<u8>,
    position: usize,
}

impl<S: Read> Iterator for Buffer<S> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        // If we are at the 0 position, get some data.
        if self.position == 0 {
            if self.read_block().ok()? == 0 {
                // Stop if we don't read any new data.
                return None;
            };
        }

        let char = self.inner[self.position];

        // If current position is going out of our block size, we reset.
        if self.position == (self.inner.len() - 1) {
            self.position = 0;
        } else {
            self.position += 1;
        }

        Some(char)
    }
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
        let byte_count = self.stream.read(&mut self.inner)?;

        // println!("Bytes read: {}", byte_count,);

        // Fill the rest of the vec with zeros if not fully filled up.
        if byte_count < self.inner.capacity() {
            self.inner[byte_count..].fill(0);
        }

        Ok(byte_count)
    }
}

pub struct Lexer<S: Read> {
    buf: Peekable<Buffer<S>>,
}

impl<S: Read> Lexer<S> {
    pub fn new(input: S) -> Self {
        Lexer {
            buf: Buffer::new(input).peekable(),
        }
    }

    fn read_number(&mut self, char: u8) -> Option<isize> {
        if !char.is_ascii_digit() {
            return None;
        };

        let mut literal_as_ascii: Vec<u8> = Vec::new();

        // Add our first character.
        literal_as_ascii.push(char);

        // Only if the next bytes are also digits, we advance.
        while let Some(digit) = self.buf.next_if(|item| item.is_ascii_digit()) {
            literal_as_ascii.push(digit);
        }

        // First convert ASCII to String, then parse to number value.
        let number_value: isize = String::from_utf8(literal_as_ascii).ok()?.parse().ok()?;

        Some(number_value)
    }
}

impl<R: Read> Iterator for Lexer<R> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        match self.buf.next()? {
            b'+' => Some(Token::Plus),
            b'-' => Some(Token::Minus),
            b'*' => Some(Token::Multiply),
            b'/' => Some(Token::Divide),
            b'^' => Some(Token::Power),
            b'(' => Some(Token::LeftParenthesis),
            b')' => Some(Token::RightParenthesis),
            char => {
                if char.is_ascii_whitespace() {
                    return Some(Token::Whitespace);
                }

                if let Some(number) = self.read_number(char) {
                    return Some(Token::Literal(Literal::Number(number)));
                }

                Some(Token::Unrecognized)

                // if self.is_letter() {}
            }
        }
    }
}

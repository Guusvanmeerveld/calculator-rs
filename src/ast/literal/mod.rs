use derive_more::Display;

mod float;
mod int;

pub use self::{float::Float, int::Int};

#[derive(Debug, PartialEq, Display)]
pub enum Literal {
    // String(String),
    Int(Int),
    Float(Float),
}

pub trait Number {
    fn as_f64(&self) -> f64;
    fn as_u32(&self) -> u32;
}

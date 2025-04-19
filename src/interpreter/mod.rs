use crate::ast::{Expression, Literal};
use crate::error::Result;

mod simple;

pub use simple::SimpleInterpreter;

pub trait Interpreter {
    fn eval(expression: Expression) -> Result<Literal>;
}

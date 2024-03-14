use std::fmt;

use crate::value::Value;

#[derive(Debug, Clone, PartialEq)]
pub enum Op {
    Plus,
    Minus,
}

#[derive(Debug, Clone, PartialEq)]
pub enum OpCode {
    Constant(Value),
    Op(Op),
    Return,
}

impl fmt::Display for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OpCode::Constant(constant) => match constant {
                Value::Float(num) => f.write_str(&num.to_string()),
                Value::Integer(num) => f.write_str(&num.to_string()),
            },
            OpCode::Op(op) => match op {
                Op::Plus => f.write_str("+"),
                Op::Minus => f.write_str("-"),
            },
            OpCode::Return => f.write_str("return"),
        }
    }
}

use std::fmt;

use crate::value::Value;

#[derive(Debug, Clone, PartialEq)]
pub enum Op {
    Plus,
    Minus,
    Multiply,
    Divide,
    EqualEqual,
    BangEqual,
    GreaterEqual,
    Greater,
    Less,
    LessEqual,
}

#[derive(Debug, Clone, PartialEq)]
pub enum OpCode {
    Constant(Value),
    Op(Op),
    Return,
    Negate,
}

impl fmt::Display for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OpCode::Constant(constant) => match constant {
                Value::Float(num) => f.write_str(&num.to_string()),
                Value::Integer(num) => f.write_str(&num.to_string()),
                Value::Bool(b) => f.write_str(&b.to_string()),
            },
            OpCode::Op(op) => match op {
                Op::Plus => f.write_str("+"),
                Op::Minus => f.write_str("-"),
                Op::Multiply => f.write_str("*"),
                Op::Divide => f.write_str("/"),
                Op::EqualEqual => f.write_str("=="),
                Op::BangEqual => f.write_str("!="),
                Op::GreaterEqual => f.write_str(">="),
                Op::Greater => f.write_str(">"),
                Op::Less => f.write_str("<"),
                Op::LessEqual => f.write_str("<="),
            },
            OpCode::Return => f.write_str("return"),
            OpCode::Negate => f.write_str("-"),
        }
    }
}

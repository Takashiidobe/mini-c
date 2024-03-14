use crate::opcode::OpCode;

pub struct Compiler {
    pub ops: Vec<OpCode>,
}

pub enum Error {
    CompileTime(String),
    Runtime(String),
}

impl Compiler {
    pub fn new(ops: &[OpCode]) -> Self {
        Self { ops: ops.to_vec() }
    }
}

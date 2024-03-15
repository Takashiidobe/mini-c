use crate::{
    opcode::{Op, OpCode},
    value::Value,
};

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Interpreter {
    ops: Vec<OpCode>,
    index: usize,
    stack: Vec<Value>,
}

impl Interpreter {
    pub fn new(ops: &[OpCode]) -> Self {
        Self {
            ops: ops.to_vec(),
            ..Default::default()
        }
    }

    pub fn interpret(&mut self) -> Value {
        while self.index < self.ops.len() {
            let op = &self.ops[self.index];
            match op {
                OpCode::Constant(value) => self.stack.push(value.clone()),
                OpCode::Return => return self.stack.last().unwrap().clone(),
                OpCode::Op(_) => self.interpret_bin_op(op.clone()),
                OpCode::Negate => {
                    let top = self.stack.pop().unwrap();
                    self.stack.push(match top {
                        Value::Float(val) => Value::Float(-val),
                        Value::Integer(val) => Value::Integer(-val),
                    });
                }
            }
            self.index += 1;
        }
        self.stack.last().unwrap().clone()
    }

    fn interpret_bin_op(&mut self, op: OpCode) {
        let b = self.stack.pop().unwrap();
        let a = self.stack.pop().unwrap();

        match (a, b) {
            (Value::Integer(a), Value::Integer(b)) => self.stack.push(match op {
                OpCode::Op(Op::Plus) => Value::from(a + b),
                OpCode::Op(Op::Minus) => Value::from(a - b),
                OpCode::Op(Op::Multiply) => Value::from(a * b),
                OpCode::Op(Op::Divide) => Value::from(a / b),
                _ => unreachable!(),
            }),
            (Value::Float(a), Value::Float(b)) => self.stack.push(match op {
                OpCode::Op(Op::Plus) => Value::from(a + b),
                OpCode::Op(Op::Minus) => Value::from(a - b),
                OpCode::Op(Op::Multiply) => Value::from(a * b),
                OpCode::Op(Op::Divide) => Value::from(a / b),
                _ => unreachable!(),
            }),
            (Value::Float(a), Value::Integer(b)) => self.stack.push(match op {
                OpCode::Op(Op::Plus) => Value::from(a + b as f64),
                OpCode::Op(Op::Minus) => Value::from(a - b as f64),
                OpCode::Op(Op::Multiply) => Value::from(a * b as f64),
                OpCode::Op(Op::Divide) => Value::from(a / b as f64),
                _ => unreachable!(),
            }),
            (Value::Integer(a), Value::Float(b)) => self.stack.push(match op {
                OpCode::Op(Op::Plus) => Value::from(a as f64 + b),
                OpCode::Op(Op::Minus) => Value::from(a as f64 - b),
                OpCode::Op(Op::Multiply) => Value::from(a as f64 * b),
                OpCode::Op(Op::Divide) => Value::from(a as f64 / b),
                _ => unreachable!(),
            }),
        }
    }
}

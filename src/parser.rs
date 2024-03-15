// find a way to go through the token stream and reinterpret it properly

use std::{collections::HashMap, ops::Add};

use crate::{
    opcode::{Op, OpCode},
    token::{Token, TokenType},
    value::Value,
};

#[derive(Default, Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum Precedence {
    #[default]
    None,
    Assignment,
    Or,
    And,
    Equality,
    Comparison,
    Term,
    Factor,
    Unary,
    Call,
    Primary,
    Top,
}

impl From<u8> for Precedence {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::None,
            1 => Self::Assignment,
            2 => Self::Or,
            3 => Self::And,
            4 => Self::Equality,
            5 => Self::Comparison,
            6 => Self::Term,
            7 => Self::Factor,
            8 => Self::Unary,
            9 => Self::Call,
            10 => Self::Primary,
            11 => Self::Top,
            _ => Self::None,
        }
    }
}

impl Add<u8> for Precedence {
    type Output = Precedence;

    fn add(self, other: u8) -> Precedence {
        Precedence::from(self.clone() as u8 + other)
    }
}

impl From<Precedence> for u8 {
    fn from(val: Precedence) -> Self {
        match val {
            Precedence::None => 0,
            Precedence::Assignment => 1,
            Precedence::Or => 2,
            Precedence::And => 3,
            Precedence::Equality => 4,
            Precedence::Comparison => 5,
            Precedence::Term => 6,
            Precedence::Factor => 7,
            Precedence::Unary => 8,
            Precedence::Call => 9,
            Precedence::Primary => 10,
            Precedence::Top => 11,
        }
    }
}

#[non_exhaustive]
#[derive(Default, Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum PrefixRule {
    #[default]
    None,
    Grouping,
    Unary,
    Number,
    Literal,
    String,
    Variable,
}

#[non_exhaustive]
#[derive(Default, Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum InfixRule {
    #[default]
    None,
    Binary,
}

#[derive(Default, Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct ParseRule {
    prefix: PrefixRule,
    infix: InfixRule,
    precedence: Precedence,
}

use lazy_static::lazy_static;

lazy_static! {
    static ref RULES: HashMap<TokenType, ParseRule> = HashMap::from([
        (
            TokenType::Minus,
            ParseRule {
                prefix: PrefixRule::Unary,
                infix: InfixRule::Binary,
                precedence: Precedence::Term,
            },
        ),
        (
            TokenType::Integer,
            ParseRule {
                prefix: PrefixRule::Number,
                ..Default::default()
            },
        ),
        (
            TokenType::Float,
            ParseRule {
                prefix: PrefixRule::Number,
                ..Default::default()
            },
        ),
        (
            TokenType::Plus,
            ParseRule {
                prefix: PrefixRule::Unary,
                infix: InfixRule::Binary,
                precedence: Precedence::Term,
            },
        ),
        (
            TokenType::Slash,
            ParseRule {
                infix: InfixRule::Binary,
                precedence: Precedence::Factor,
                ..Default::default()
            },
        ),
        (
            TokenType::Star,
            ParseRule {
                infix: InfixRule::Binary,
                precedence: Precedence::Factor,
                ..Default::default()
            },
        ),
        (
            TokenType::LeftParen,
            ParseRule {
                prefix: PrefixRule::Grouping,
                ..Default::default()
            },
        ),
    ]);
}

fn get_rule(token_type: &TokenType) -> ParseRule {
    RULES.get(token_type).cloned().unwrap_or_default()
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Parser {
    tokens: Vec<Token>,
    index: usize,
    ops: Vec<OpCode>,
}

impl Parser {
    pub fn new(tokens: &[Token]) -> Self {
        Self {
            tokens: tokens.to_vec(),
            ..Default::default()
        }
    }

    pub fn parse(&mut self) -> Vec<OpCode> {
        while self.curr().r#type != TokenType::Eof {
            self.expression();
        }
        self.emit_return();
        self.ops.clone()
    }

    fn expression(&mut self) {
        self.parse_precedence(Precedence::Assignment);
    }

    fn grouping(&mut self) {
        self.expression();
        self.consume(&TokenType::RightParen, "Expect ')' after expression.")
    }

    fn consume(&mut self, token_type: &TokenType, message: &str) {
        if self.curr().r#type == *token_type {
            self.advance();
            return;
        }
        panic!("{}", message);
    }

    fn binary(&mut self) {
        let prev = self.prev();
        let operator_type = prev.r#type;

        let rule_precedence = get_rule(&operator_type).precedence + 1;

        self.parse_precedence(rule_precedence);

        match operator_type {
            TokenType::Integer | TokenType::Float => {
                self.ops.push(OpCode::Constant(prev.value.unwrap()))
            }
            TokenType::Plus => self.emit_byte(OpCode::Op(Op::Plus)),
            TokenType::Minus => self.emit_byte(OpCode::Op(Op::Minus)),
            TokenType::Star => self.emit_byte(OpCode::Op(Op::Multiply)),
            TokenType::Slash => self.emit_byte(OpCode::Op(Op::Divide)),
            TokenType::Eof => self.emit_byte(OpCode::Return),
            _ => panic!("Reached an error"),
        }
    }

    fn unary(&mut self) {
        let operator_type = self.prev().r#type;

        self.parse_precedence(Precedence::Unary);

        match operator_type {
            TokenType::Minus => self.emit_byte(OpCode::Negate),
            TokenType::Plus => {}
            _ => unreachable!(),
        }
    }

    fn parse_precedence(&mut self, precedence: Precedence) {
        self.advance();
        let prefix_rule = get_rule(&self.prev().r#type).prefix;

        match prefix_rule {
            PrefixRule::Number => self.number(),
            PrefixRule::Grouping => self.grouping(),
            PrefixRule::Unary => self.unary(),
            PrefixRule::None => panic!("Expected expression"),
            _ => unreachable!(),
        }

        while precedence <= get_rule(&self.curr().r#type).precedence {
            self.advance();
            let infix_rule = get_rule(&self.prev().r#type).infix;

            match infix_rule {
                InfixRule::Binary => self.binary(),
                InfixRule::None => unreachable!(),
            }
        }
    }

    fn advance(&mut self) {
        if self.curr().r#type != TokenType::Error {
            self.index += 1;
        }
    }

    fn curr(&self) -> Token {
        self.tokens[self.index].clone()
    }

    fn prev(&self) -> Token {
        self.tokens[self.index - 1].clone()
    }

    fn number(&mut self) {
        if let (TokenType::Integer | TokenType::Float, Some(val)) =
            (&self.prev().r#type, &self.prev().value)
        {
            self.emit_constant(val.clone())
        }
    }

    fn emit_constant(&mut self, value: Value) {
        self.emit_byte(OpCode::Constant(value));
    }

    fn end_compiler(&mut self) {
        self.emit_return();
    }

    fn emit_return(&mut self) {
        self.emit_byte(OpCode::Return);
    }

    fn emit_byte(&mut self, opcode: OpCode) {
        self.ops.push(opcode);
    }

    fn emit_bytes(&mut self, bytes: &[OpCode]) {
        for byte in bytes {
            self.emit_byte(byte.clone());
        }
    }
}

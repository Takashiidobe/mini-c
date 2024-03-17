use serde::{Deserialize, Serialize};

use crate::value::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TokenInfo {
    pub start: usize,
    pub length: usize,
    pub line: usize,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Eq, PartialOrd, Ord, Hash)]
pub enum TokenType {
    Integer,
    Float,
    Plus,
    Minus,
    Star,
    Slash,
    Error,
    LeftParen,
    RightParen,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    EqualEqual,
    BangEqual,
    Equal,
    #[default]
    Eof,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Token {
    pub r#type: TokenType,
    pub value: Option<Value>,
    pub info: TokenInfo,
}

impl From<char> for TokenType {
    fn from(value: char) -> Self {
        match value {
            '(' => TokenType::LeftParen,
            ')' => TokenType::RightParen,
            '-' => TokenType::Minus,
            '+' => TokenType::Plus,
            '/' => TokenType::Slash,
            '*' => TokenType::Star,
            '>' => TokenType::Greater,
            '<' => TokenType::Less,
            '=' => TokenType::Equal,
            _ => panic!("Cannot parse from char: {}", value),
        }
    }
}

impl From<&str> for TokenType {
    fn from(value: &str) -> Self {
        match value {
            "(" => TokenType::LeftParen,
            ")" => TokenType::RightParen,
            "-" => TokenType::Minus,
            "+" => TokenType::Plus,
            "/" => TokenType::Slash,
            "*" => TokenType::Star,
            "!=" => TokenType::BangEqual,
            "==" => TokenType::EqualEqual,
            ">" => TokenType::Greater,
            ">=" => TokenType::GreaterEqual,
            "<" => TokenType::Less,
            "<=" => TokenType::LessEqual,
            _ => panic!("Couldn't parse from str: {}", value),
        }
    }
}

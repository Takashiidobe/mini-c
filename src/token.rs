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
    #[default]
    Eof,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Token {
    pub r#type: TokenType,
    pub value: Option<Value>,
    pub info: TokenInfo,
}
